use crate::asset::sound::dat::finetune::FineTune;

use super::{pattern_event::*, t_instrument::*, t_song::*};
use std::ops::Deref;

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
            for row in pattern.deref() {
                // Update channels
                for (c, event) in row.iter().enumerate() {
                    let channel = &mut channels[c];

                    // Process note
                    if let Some(note) = event.note {
                        channel.note = note;
                    }
                    if let Some(instrument) = &event.instrument {
                        channel.instrument = Some(instrument);
                        channel.sample_position = 0;
                    }
                    if let Some(volume) = event.volume {
                        channel.volume = volume as f32 / u8::MAX as f32;
                    }

                    // Process effects
                    for effect in event.effects.iter().flatten() {
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
        // TODO(nenikitov): Figure out what constant `24` means (maybe `6` default speed * `4` beats/measure???)
        60.0 / (bpm * 24 / speed) as f32
    }
}

#[derive(Default)]
struct Channel<'a> {
    instrument: Option<&'a PatternEventInstrumentKind>,

    sample_position: usize,

    volume: f32,
    note: PatternEventNote,
}

// TODO(nenikitov): Double check that the game actually uses C_5 as a base note for all samples
const BASE_NOTE: FineTune = FineTune::new_from_note(60);

impl<'a> Channel<'a> {
    fn tick(&mut self, duration: usize) -> Sample {
        if let Some(instrument) = self.instrument
            && let PatternEventNote::On(note) = self.note
            && let PatternEventInstrumentKind::Predefined(instrument) = instrument
            && let TInstrumentSampleKind::Predefined(sample) = &instrument.samples[note.note() as usize]
        {
            let pitch_factor = (BASE_NOTE - sample.finetune) / note;

            let duration_scaled = (duration as f32 / pitch_factor).round() as usize;

            let data = sample
                .sample_beginning()
                .iter()
                .chain(sample.sample_loop().iter().cycle())
                .skip(self.sample_position)
                .take(duration_scaled)
                .copied()
                .collect::<Vec<_>>();

            self.sample_position += duration_scaled;

            let factor = duration as f32 / data.len() as f32;
            data.volume(self.volume).pitch_with_time_stretch(factor)
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
    fn pitch_with_time_stretch(self, note: f32) -> Sample;
    fn volume(self, volume: f32) -> Sample;
}

impl SoundEffect for Sample {
    fn pitch_with_time_stretch(self, factor: f32) -> Sample {
        let len = (self.len() as f32 * factor).floor() as usize;

        (0..len)
            .map(|i| self[(i as f32 / factor).floor() as usize])
            .collect()
    }

    fn volume(self, volume: f32) -> Sample {
        self.into_iter()
            .map(|s| (s as f32 * volume) as i16)
            .collect()
    }
}
