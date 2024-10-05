use std::{collections::HashMap, rc::Rc};

use itertools::Itertools;

use super::{
    finetune::FineTune, pattern_effect::*, pattern_event::*, t_instrument::*, t_song::TSong,
};
use crate::asset::sound::sample::Sample;

trait AudioSamplePoint {
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

#[derive(Default, Clone)]
struct PlayerChannelNote {
    finetune: Option<FineTune>,
    finetune_initial: Option<FineTune>,
    on: bool,
}

#[derive(Default, Clone)]
struct PlayerChannel {
    instrument: Option<Rc<TInstrument>>,
    sample: Option<Rc<TSample>>,
    note: PlayerChannelNote,
    volume: f32,
    effects: [Option<PatternEffect>; 2],
    effects_memory: HashMap<PatternEffectMemoryKey, PatternEffect>,
    note_delay: usize,

    previous: Option<(Box<Self>, f64)>,

    pos_sample: f64,
    pos_volume_envelope: usize,
    direction: PlaybackDirection,
}

impl PlayerChannel {
    // Too large of a time and samples will audibly blend and play 2 notes at the same time, which sounds weird.
    // Too little and transitions between notes will click.
    // This is 800 microseconds, which is a bit of an arbitrary value that i found sounds nice.
    // It amounts to:
    // - 13 samples at 16000
    // - 35 samples at 44100
    // - 38 samples at 48000
    const SAMPLE_BLEND: f64 = 0.0008;

    fn note_cut(&mut self) {
        self.volume = 0.;
    }

    fn pos_reset(&mut self) {
        self.pos_sample = 0.;
        self.pos_volume_envelope = 0;
        self.direction = PlaybackDirection::Forwards;
    }

    fn generate_sample<T: AudioSamplePoint>(&mut self, step: f64) -> T {
        let current_sample = if let Some(instrument) = &self.instrument
            && let Some(sample) = &self.sample
            && let Some(note) = self.note.finetune
            && self.note_delay == 0
            && let Some(value) = sample.get(self.pos_sample)
        {
            let pitch_factor = (note + sample.finetune).pitch_factor();
            let step = step / pitch_factor;
            self.pos_sample += match self.direction {
                PlaybackDirection::Forwards => step,
                PlaybackDirection::Backwards => -step,
            };

            let volume_envelope = match &instrument.volume {
                TInstrumentVolume::Envelope(envelope) => {
                    if self.note.on {
                        envelope
                            .volume_beginning()
                            .get(self.pos_volume_envelope)
                            .cloned()
                            .unwrap_or(envelope.volume_loop())
                    } else {
                        envelope
                            .volume_end()
                            .get(self.pos_volume_envelope.saturating_sub(
                                envelope.volume_beginning().len().saturating_sub(1),
                            ))
                            .cloned()
                            .unwrap_or(0.)
                    }
                }
                TInstrumentVolume::Constant(_) => 1.,
            };

            value.into_normalized_f32() * volume_envelope * self.volume
        } else {
            0.
        };

        let current_sample = if let Some((previous, position)) = &mut self.previous {
            let factor = (*position / Self::SAMPLE_BLEND) as f32;
            let previous_sample = previous.generate_sample::<T>(step).into_normalized_f32();

            *position += step;
            if *position >= Self::SAMPLE_BLEND {
                self.previous = None
            }

            previous_sample + factor * (current_sample - previous_sample)
        } else {
            current_sample
        };

        T::from_normalized_f32(current_sample)
    }

    fn change_instrument(&mut self, instrument: Option<Rc<TInstrument>>) {
        // In tracker music, every instrument change is a state reset
        // Previous state is kept to subtly blend in notes to remove clicks.

        // Disregard previous state before `self.clone` so we don't have a fully recursive structure.
        self.previous = None;
        self.previous = Some((Box::new(self.clone()), 0.));

        if let Some(instrument) = instrument {
            self.instrument = Some(instrument);
            self.pos_reset();
        } else {
            // TODO(nenikitov): Idk honestly, figure this out
            self.note_cut();
            self.instrument = None;
            self.sample = None;
        }
    }

    fn change_note(&mut self, note: PatternEventNote) {
        if let Some(instrument) = &self.instrument {
            match note {
                PatternEventNote::Off => {
                    self.note.on = false;
                }
                PatternEventNote::On(note) => {
                    self.note.finetune = Some(note);
                    self.note.finetune_initial = Some(note);
                    self.note.on = true;
                    self.sample = instrument.samples[note.note() as usize].clone();
                }
            }
        } else {
            // TODO(nenikitov): Idk honestly, figure this out
            self.note_cut();
        }
    }

    fn change_volume(&mut self, volume: PatternEventVolume) {
        self.volume = match volume {
            PatternEventVolume::Sample => {
                if let Some(sample) = &self.sample {
                    sample.volume
                } else {
                    self.note_cut();
                    return;
                }
            }
            PatternEventVolume::Value(volume) => volume,
        };
    }

    fn change_effect(&mut self, i: usize, effect: PatternEffect) {
        // Recall from memory
        let effect = if let Some(key) = effect.memory_key() {
            if !effect.is_empty() {
                self.effects_memory.insert(key, effect);
            }

            self.effects_memory[&key]
        } else {
            effect
        };
        self.effects[i] = Some(effect);
    }

    fn advance_envelopes(&mut self) {
        if let Some(instrument) = &self.instrument
            && let TInstrumentVolume::Envelope(envelope) = &instrument.volume
        {
            if (self.note.on
                && if let Some(sustain) = envelope.sustain {
                    self.pos_volume_envelope < sustain
                } else {
                    true
                })
                || !self.note.on
            {
                self.pos_volume_envelope += 1;
            }
        }
    }
}

struct Player<'a> {
    song: &'a TSong,
    sample_rate: usize,

    time_in_tick: f64,

    pos_loop: usize,
    pos_pattern: usize,
    pos_row: usize,
    pos_tick: usize,

    tempo: usize,
    bpm: usize,
    volume_global: f32,
    volume_amplification: f32,

    channels: Vec<PlayerChannel>,
}

impl<'a> Player<'a> {
    fn new(song: &'a TSong, sample_rate: usize) -> Self {
        Self {
            song,
            sample_rate,
            time_in_tick: 0.,
            pos_loop: 0,
            pos_pattern: 0,
            pos_row: 0,
            pos_tick: 0,
            tempo: song.speed as usize,
            bpm: song.bpm as usize,
            volume_global: 1.,
            volume_amplification: 0.25,
            channels: (0..song.orders[0][0].len())
                .map(|_| PlayerChannel::default())
                .collect(),
        }
    }

    fn generate_sample<S: AudioSamplePoint>(&mut self) -> S {
        if self.time_in_tick <= 0. {
            self.tick();
        }
        let step = 1. / self.sample_rate as f64;
        self.time_in_tick -= step;

        let sample = self
            .channels
            .iter_mut()
            .map(|c| c.generate_sample::<S>(step))
            .map(|c| c.into_normalized_f32())
            //.enumerate()
            //.filter_map(|(i, s)| (i == 4).then_some(s))
            .sum::<f32>();
        S::from_normalized_f32(sample * self.volume_global * self.volume_amplification)
    }

    fn tick(&mut self) {
        if self.pos_tick == 0 {
            self.row();
        }

        for channel in self.channels.iter_mut() {
            channel.advance_envelopes();

            for effect in channel.effects.iter().flatten() {
                use PatternEffect as E;
                match *effect {
                    // Tick effects
                    E::Volume(Volume::Slide(Some(volume))) => {
                        channel.volume = (channel.volume + volume).clamp(0., 1.);
                    }
                    E::Porta(Porta::Tone(Some(step))) => {
                        if let Some(finetune_initial) = channel.note.finetune_initial {
                            channel.note.finetune = channel.note.finetune.map(|finetune| {
                                use std::cmp::Ordering;
                                match finetune.cmp(&finetune_initial) {
                                    Ordering::Less => {
                                        FineTune::min(finetune + step, finetune_initial)
                                    }
                                    Ordering::Greater => {
                                        FineTune::max(finetune - step, finetune_initial)
                                    }
                                    Ordering::Equal => finetune,
                                }
                            });
                        }
                    }
                    E::Porta(Porta::Slide {
                        finetune: Some(finetune),
                        ..
                    }) => {
                        channel.note.finetune = channel
                            .note
                            .finetune
                            .map(|f| (f + finetune).clamp(FineTune::new(0), FineTune::new(15488)));
                    }
                    E::NoteDelay(_) => {
                        channel.note_delay = channel.note_delay.saturating_sub(1);
                    }
                    // Noops - no tick
                    E::Volume(Volume::Set(..))
                    | E::Volume(Volume::Bump { .. })
                    | E::Porta(Porta::Tone(..))
                    | E::Porta(Porta::Bump { .. })
                    | E::Speed(..)
                    | E::GlobalVolume(..)
                    | E::SampleOffset(..)
                    | E::PlaybackDirection(..) => {}
                    // TODO(nenikitov): Unemplemented
                    E::Dummy(..) => {}
                    // Unreachable because memory has to be initialized
                    E::Volume(Volume::Slide(None))
                    | E::Porta(Porta::Slide { finetune: None, .. }) => {
                        unreachable!("Effects should have their memory initialized")
                    }
                }
            }
        }

        self.pos_tick += 1;

        if self.pos_tick >= self.tempo {
            self.pos_tick = 0;
            self.pos_row += 1;
        }
        if let Some(pattern) = self.song.orders.get(self.pos_pattern)
            && self.pos_row >= pattern.len()
        {
            self.pos_row = 0;
            self.pos_pattern += 1;
        };
        if self.pos_pattern >= self.song.orders.len() {
            self.pos_pattern = self.song.restart_order as usize;
            self.pos_loop += 1;
        }

        self.time_in_tick += 2.5 / (self.bpm as f64);
    }

    fn row(&mut self) {
        let Some(row) = self
            .song
            .orders
            .get(self.pos_pattern)
            .and_then(|p| p.get(self.pos_row))
        else {
            return;
        };

        for (channel, event) in self.channels.iter_mut().zip_eq(row) {
            if let Some(instrument) = &event.instrument {
                channel.change_instrument(instrument.clone());
            }

            if let Some(note) = &event.note {
                channel.change_note(note.clone());
            }

            if let Some(volume) = &event.volume {
                channel.change_volume(volume.clone());
            }

            for (i, effect) in event
                .effects
                .iter()
                .enumerate()
                .filter_map(|(i, e)| e.map(|e| (i, e)))
            {
                channel.change_effect(i, effect.clone());

                use PatternEffect as E;
                match channel.effects[i].unwrap() {
                    // Init effects
                    E::Speed(Speed::Bpm(bpm)) => {
                        self.bpm = bpm;
                    }
                    E::Speed(Speed::TicksPerRow(ticks_per_row)) => {
                        self.tempo = ticks_per_row;
                    }
                    E::GlobalVolume(volume) => {
                        self.volume_global = volume;
                    }
                    E::Volume(Volume::Set(volume)) => {
                        channel.volume = volume;
                    }
                    E::Volume(Volume::Bump {
                        volume: Some(volume),
                        ..
                    }) => {
                        channel.volume = (channel.volume + volume).clamp(0., 1.);
                    }
                    E::Porta(Porta::Bump {
                        finetune: Some(finetune),
                        ..
                    }) => {
                        channel.note.finetune = channel.note.finetune.map(|f| f + finetune);
                    }
                    E::PlaybackDirection(direction) => {
                        channel.direction = direction;
                        if let Some(sample) = &channel.sample
                            && direction == PlaybackDirection::Backwards
                        {
                            channel.pos_sample = sample.data.len_seconds() as f64
                        }
                    }
                    E::SampleOffset(Some(offset)) => {
                        // TODO(nenikitov): Remove this hardcoded value
                        channel.pos_sample = 1. / 16_000. * offset as f64;
                    }
                    E::NoteDelay(delay) => {
                        channel.note_delay = delay;
                    }
                    // Noops - no init
                    E::Volume(Volume::Slide(..)) => {}
                    E::Porta(Porta::Tone(..)) => {}
                    E::Porta(Porta::Slide { .. }) => {}
                    // TODO(nenikitov): To implement
                    E::Dummy(..) => {}
                    // Unreachable because memory has to be initialized
                    E::Volume(Volume::Bump { volume: None, .. })
                    | E::Porta(Porta::Tone(None))
                    | E::Porta(Porta::Bump { finetune: None, .. })
                    | E::SampleOffset(None) => {
                        unreachable!("Effects should have their memory initialized")
                    }
                }
            }
        }
    }
}

pub trait TSongMixerNew {
    fn mix_new(&self) -> Sample<i16, 1>;
}

impl TSongMixerNew for TSong {
    fn mix_new(&self) -> Sample<i16, 1> {
        const SAMPLE_RATE: usize = 16000;

        let mut player = Player::new(self, SAMPLE_RATE);

        let samples: Vec<_> =
            std::iter::from_fn(|| (player.pos_loop == 0).then(|| player.generate_sample::<i16>()))
                .map(|s| [s])
                .collect();

        Sample {
            data: samples,
            sample_rate: player.sample_rate,
        }
    }
}
