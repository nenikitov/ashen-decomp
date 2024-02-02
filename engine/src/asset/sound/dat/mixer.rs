use crate::asset::sound::dat::t_song::{NoteState, PatternEffectKind, TPatternFlags};

use super::{
    t_instrument::TInstrument,
    t_song::{TPattern, TSong},
};

type SamplePoint = i16;
type Sample = Vec<SamplePoint>;

pub trait TSongMixer {
    fn mix(&self, restart: bool) -> Sample;
}

impl TSongMixer for TSong {
    fn mix(&self, restart: bool) -> Sample {
        TSongMixerUtils::mix(
            self,
            if restart {
                self.restart_order as usize
            } else {
                0
            },
        )
    }
}

trait TSongMixerUtils {
    const SAMPLE_RATE: usize = 16000;
    const CHANNEL_COUNT: usize = 1;

    fn mix(&self, start: usize) -> Sample;

    fn ticks_per_beat(bpm: usize, speed: usize) -> f32;
}

impl TSongMixerUtils for TSong {
    fn mix(&self, start: usize) -> Sample {
        let mut m = Mixer::new();

        let mut channels: Vec<_> = (0..self.patterns[0].len())
            .map(|_| Channel::default())
            .collect();

        let mut offset: usize = 0;
        let mut bpm = self.bpm;

        for pattern in &self.orders[start..] {
            let pattern = &self.patterns[*pattern as usize];
            for row in pattern {
                // Update channels
                for (c, event) in row.iter().enumerate() {
                    let Some(event) = event else { continue };
                    let channel = &mut channels[c];

                    // Process note
                    match event.note {
                        NoteState::None => {}
                        NoteState::On(_) => {
                            // TODO(nenikitov): This will become huge with more effects, this will need to be refactored
                            channel.note = event.note;
                            channel.instrument = Some(&self.instruments[event.instrument as usize]);
                            channel.volume = event.volume as f32 / 255.0;
                            channel.sample_posion = 0;
                        }
                        NoteState::Off => {
                            // TODO(nenikitov): This is repeated from `NoteState::On`, somehow refactor
                            channel.note = event.note;
                        }
                    }

                    // Process effects
                    let effects = [
                        if event.flags.contains(TPatternFlags::DoEffect1) {
                            Some(&event.effect_1)
                        } else {
                            None
                        },
                        if event.flags.contains(TPatternFlags::DoEffect2) {
                            Some(&event.effect_2)
                        } else {
                            None
                        },
                    ];
                    for effect in effects.into_iter().flatten() {
                        match effect.kind {
                            // TODO(nenikitov): Add effects
                            PatternEffectKind::Speed => {
                                bpm = effect.value;
                            }
                            _ => {}
                        }
                    }
                }

                // Mix current tick
                let sample_length = Self::ticks_per_beat(bpm as usize, self.speed as usize)
                    * Self::SAMPLE_RATE as f32;
                let sample_length = sample_length as usize;

                for c in &channels {
                    m.add_sample(&c.tick(sample_length), offset);
                }

                // Advance to next tick
                offset += sample_length;
            }
        }

        m.mix()
    }

    fn ticks_per_beat(bpm: usize, speed: usize) -> f32 {
        (bpm * speed) as f32 / 60.0
    }
}

#[derive(Default)]
struct Channel<'a> {
    instrument: Option<&'a TInstrument>,
    sample_posion: usize,

    volume: f32,
    note: NoteState,
}

impl<'a> Channel<'a> {
    fn tick(&self, duration: usize) -> Sample {
        todo!()
    }

    fn play_event(&mut self, event: &TPattern, instruments: &'a [TInstrument]) {
        if event.note != NoteState::None {
            *self = Self::default();
        }

        if let NoteState::On(note) = event.note {
            self.instrument = Some(&instruments[event.instrument as usize]);
        }
    }
}

// TODO(nenikitov): Remove this code when new mixer is done

pub struct Mixer {
    samples: Sample,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, sample: &Sample, offset: usize) {
        let new_len = offset + sample.len();
        if new_len > self.samples.len() {
            self.samples.resize(new_len, 0);
        }

        for (i, s) in sample.iter().enumerate() {
            let i = i + offset;
            if i < self.samples.len() {
                self.samples[i] = self.samples[i].saturating_add(*s);
            }
        }
    }

    pub fn mix(self) -> Sample {
        self.samples
    }
}

pub trait SoundEffect {
    fn pitch(self, note: u8) -> Sample;
    fn volume(self, volume: f32) -> Sample;
}

impl SoundEffect for Sample {
    fn pitch(self, note: u8) -> Sample {
        todo!("(nenikitov): Figure out how this work")
    }

    fn volume(self, volume: f32) -> Sample {
        self.into_iter()
            .map(|s| (s as f32 * volume) as i16)
            .collect()
    }
}

fn note_frequency(note: u8) {}
