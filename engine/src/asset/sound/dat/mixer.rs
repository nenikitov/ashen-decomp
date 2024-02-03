use crate::asset::sound::dat::t_song::{NoteState, PatternEffectKind, TPatternFlags};

use super::{
    t_instrument::{TInstrument, TInstrumentSampleKind},
    t_song::TSong,
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

    fn seconds_per_tick(bpm: usize, speed: usize) -> f32;
}

impl TSongMixerUtils for TSong {
    fn mix(&self, start: usize) -> Sample {
        let mut m = Mixer::new();

        let mut channels: Vec<_> = (0..self.patterns[0][0].len())
            .map(|_| Channel::default())
            .collect();

        let mut offset = 0;
        let mut sample_length_fractional = 0.0;
        let mut bpm = self.bpm;
        let mut speed = self.speed;

        for pattern in &self.orders[start..] {
            let pattern = &self.patterns[*pattern as usize];
            for row in pattern {
                // Update channels
                for (c, event) in row.iter().enumerate() {
                    let Some(event) = event else { continue };
                    let channel = &mut channels[c];

                    // Process note
                    if event.flags.contains(TPatternFlags::ChangeNote) {
                        channel.note = event.note;
                    }
                    if event.flags.contains(TPatternFlags::ChangeInstrument) {
                        if event.instrument != 255 {
                            channel.instrument = Some(&self.instruments[event.instrument as usize]);
                        }
                        channel.sample_posion = SamplePosition::default();
                    }
                    if event.flags.contains(TPatternFlags::ChangeVolume) {
                        channel.volume = event.volume as f32 / u8::MAX as f32;
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
                                if effect.value >= 0x20 {
                                    bpm = effect.value;
                                } else {
                                    speed = effect.value;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Mix current tick
                let sample_length = Self::seconds_per_tick(bpm as usize, speed as usize)
                    * Self::SAMPLE_RATE as f32
                    + sample_length_fractional;
                sample_length_fractional = sample_length - sample_length.floor();
                let sample_length = sample_length as usize;

                for c in &mut channels {
                    m.add_sample(&c.tick(sample_length), offset);
                }

                // Advance to next tick
                offset += sample_length;
            }
        }

        m.mix()
    }

    fn seconds_per_tick(bpm: usize, speed: usize) -> f32 {
        60.0 / (bpm * speed) as f32
    }
}

#[derive(PartialEq)]
enum SamplePosition {
    Beginning,
    Loop(isize),
}

impl Default for SamplePosition {
    fn default() -> Self {
        Self::Beginning
    }
}

#[derive(Default)]
struct Channel<'a> {
    instrument: Option<&'a TInstrument>,
    sample_posion: SamplePosition,

    volume: f32,
    note: NoteState,
}

impl<'a> Channel<'a> {
    // TODO(nenikitov): Don not pass `samples`, it should somehow be stored in the instrument
    fn tick(&mut self, duration: usize) -> Sample {
        if let Some(instrument) = self.instrument
            && let NoteState::On(note) = self.note
            && let TInstrumentSampleKind::Predefined(sample) = &instrument.samples[note as usize]
        {
            let sample = sample.sample_full().to_vec().volume(self.volume);
            self.note = NoteState::None;
            sample
        } else {
            vec![]
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

    pub fn add_sample(&mut self, sample: &[i16], offset: usize) {
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
