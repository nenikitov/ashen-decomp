use std::{iter::Sum, rc::Rc};

use super::{
    finetune::FineTune,
    pattern_event::{PatternEventNote, PatternEventVolume},
    t_instrument::{TInstrument, TSample},
    t_song::TSong,
};
use crate::asset::sound::sample::Sample;

trait AudioSamplePoint: Sum {
    type Bytes: IntoIterator<Item = u8>;

    fn into_normalized_f32(&self) -> f32;
    fn from_normalized_f32(value: f32) -> Self;
    fn into_bytes(&self) -> Self::Bytes;
}

impl AudioSamplePoint for i16 {
    type Bytes = [u8; 2];

    fn into_normalized_f32(&self) -> f32 {
        if *self < 0 {
            -(*self as f32 / Self::MIN as f32)
        } else {
            (*self as f32 / Self::MAX as f32)
        }
    }

    fn from_normalized_f32(value: f32) -> Self {
        if value < 0. {
            -(value * i16::MIN as f32) as i16
        } else {
            (value * i16::MAX as f32) as i16
        }
    }

    fn into_bytes(&self) -> Self::Bytes {
        self.to_le_bytes()
    }
}

struct PlayerChannelNote {
    finetune: FineTune,
    finetune_initial: FineTune,
    on: bool,
}

#[derive(Default)]
struct PlayerChannel {
    instrument: Option<Rc<TInstrument>>,
    sample: Option<Rc<TSample>>,
    note: Option<PlayerChannelNote>,
    volume: Option<PatternEventVolume>,
    pos_sample: f64,
    pos_volume_envelope: usize,
}

impl PlayerChannel {
    fn note_cut(&mut self) {
        self.volume = Some(PatternEventVolume::Value(0.));
    }

    fn note_trigger(&mut self) {
        self.pos_sample = 0f64;
        self.pos_volume_envelope = 0;
    }

    fn generate_sample<T: AudioSamplePoint>(&mut self, sample_length: f64) -> T {
        if let Some(instrument) = &self.instrument
            && let Some(sample) = &self.sample
            && let Some(volume) = &self.volume
            && let Some(note) = &self.note
        {
            let value = sample.get(self.pos_sample).into_normalized_f32();
            let volume = match volume {
                PatternEventVolume::Sample => sample.volume,
                PatternEventVolume::Value(volume) => *volume,
            };

            let pitch_factor = (note.finetune + sample.finetune).pitch_factor();
            self.pos_sample += sample_length / pitch_factor;

            T::from_normalized_f32(value * volume)
        } else {
            T::from_normalized_f32(0f32)
        }
    }
}

struct Player<'a> {
    song: &'a TSong,

    time_in_tick: f64,

    pos_loop: usize,
    pos_pattern: usize,
    pos_row: usize,
    pos_tick: usize,

    tempo: usize,
    bpm: usize,
    volume_global: f32,

    channels: Vec<PlayerChannel>,
}

impl<'a> Player<'a> {
    fn new(song: &'a TSong) -> Self {
        Self {
            song,
            time_in_tick: 0.,
            pos_loop: 0,
            pos_pattern: 0,
            pos_row: 0,
            pos_tick: 0,
            tempo: song.speed as usize,
            bpm: song.bpm as usize,
            volume_global: 0.25,
            channels: (0..song.patterns[0][0].len())
                .map(|_| PlayerChannel::default())
                .collect(),
        }
    }

    fn generate_sample<S: AudioSamplePoint>(&mut self, sample_rate: usize) -> S {
        if self.time_in_tick <= 0f64 {
            self.tick();
        }
        let sample_length = 1. / sample_rate as f64;
        self.time_in_tick -= sample_length;

        let sample = self
            .channels
            .iter_mut()
            .map(|c| c.generate_sample::<S>(sample_length))
            .map(|c| c.into_normalized_f32())
            .sum::<f32>();
        S::from_normalized_f32(sample * self.volume_global)
    }

    fn tick(&mut self) {
        if self.pos_tick == 0 {
            self.row();
        }

        self.pos_tick += 1;

        if self.pos_tick >= self.tempo {
            self.pos_tick = 0;
            self.pos_row += 1;
        }
        if let Some(pattern) = self.song.patterns.get(self.pos_pattern)
            && self.pos_row >= pattern.len()
        {
            self.pos_pattern += 1;
            self.pos_row = 0;
        };
        if self.pos_pattern >= self.song.patterns.len() {
            self.pos_loop += 1;
            self.pos_pattern = self.song.restart_order as usize;
        }

        self.time_in_tick += 2.5 / (self.bpm as f64);
    }

    fn row(&mut self) {
        let Some(row) = self
            .song
            .patterns
            .get(self.pos_pattern)
            .and_then(|p| p.get(self.pos_row))
        else {
            return;
        };

        for (channel, event) in self.channels.iter_mut().zip(row) {
            if let Some(instrument) = &event.instrument {
                if let Some(instrument) = instrument {
                    channel.instrument = Some(instrument.clone());
                } else {
                    // TODO(nenikitov): Idk honestly, figure this out
                    channel.note_cut();
                    channel.instrument = None;
                    channel.sample = None;
                }
            }

            if let Some(note) = &event.note {
                if let Some(instrument) = &channel.instrument {
                    match note {
                        PatternEventNote::Off => {
                            if let Some(note) = &mut channel.note {
                                note.on = false;
                            }
                        }
                        PatternEventNote::On(note) => {
                            channel.note = Some(PlayerChannelNote {
                                finetune: *note,
                                finetune_initial: *note,
                                on: true,
                            });
                            channel.sample = instrument.samples[note.note() as usize].clone();
                            channel.note_trigger();
                        }
                    }
                } else {
                    // TODO(nenikitov): Idk honestly, figure this out
                    channel.note_cut();
                }
            }

            if let Some(volume) = &event.volume {
                channel.volume = Some(volume.clone());
            }

            // TODO(nenikitov): Do effects
        }
    }
}

pub trait TSongMixerNew {
    fn mix_new(&self) -> Sample<i16, 1>;
}

impl TSongMixerNew for TSong {
    fn mix_new(&self) -> Sample<i16, 1> {
        let mut player = Player::new(self);

        let samples: Vec<_> = std::iter::from_fn(|| {
            (player.pos_loop == 0).then(|| player.generate_sample::<i16>(48000))
        })
        .map(|s| [s])
        .collect();

        Sample {
            data: samples,
            sample_rate: 48000,
        }
    }
}
