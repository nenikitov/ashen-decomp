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
    const SAMPLE_RATE: usize = 48000;
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
            && let TInstrumentSampleKind::Predefined(sample) =
                &instrument.samples[note.note() as usize]
        {
            let pitch_factor = (BASE_NOTE - sample.finetune) / note;

            let duration_scaled = (duration as f32 / pitch_factor).round() as usize;

            let data = sample
                .sample_beginning()
                .iter()
                .chain(sample.sample_loop().iter().cycle())
                .skip(self.sample_position)
                // TODO(nenikitov): This `+1` is necessary for linear interpolation.
                // If not present, linear interpolation can produce clicks.
                // If interpolation is reverted, this magic shouldn't be here
                .take(duration_scaled + 1)
                .copied()
                .collect::<Vec<_>>();

            self.sample_position += duration_scaled;

            // TODO(nenikitov): Same here, these `+1` and `pop` are necessary for linear interpolation.
            // If interpolation is reverted, this magic shouldn't be here
            let pitch_factor = (duration + 1) as f32 / data.len() as f32;
            let mut data = data
                .volume(self.volume)
                .pitch_with_time_stretch(pitch_factor, true);
            if (data.len() != duration) {
                data.pop();
            }

            data
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
    fn volume(self, volume: f32) -> Self;
    fn pitch_with_time_stretch(self, factor: f32, interpolate: bool) -> Self;
}

impl SoundEffect for Sample {
    fn volume(self, volume: f32) -> Self {
        self.into_iter()
            .map(|s| (s as f32 * volume) as i16)
            .collect()
    }

    fn pitch_with_time_stretch(self, factor: f32, interpolate: bool) -> Self {
        // TODO(nenikitov): Linear interpolation sounds nicer and less crusty, but
        // Introduces occasional clicks.
        // Maybe it should be removed.
        let len = (self.len() as f32 * factor as f32).round() as usize;

        (0..len)
            .map(|i| {
                let frac = i as f32 / factor;
                let index = (frac).floor() as usize;
                let frac = if interpolate {
                    frac - index as f32
                } else {
                    0.0
                };

                let sample_1 = self[index];
                let sample_2 = if index + 1 < self.len() {
                    self[index + 1]
                } else {
                    self[index]
                };

                ((1.0 - frac) * sample_1 as f32 + frac * sample_2 as f32) as i16
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sound_effect_volume_works() {
        assert_eq!(
            vec![-10, 20, 40, 30, -78],
            vec![-10, 20, 40, 30, -78].volume(1.0),
        );
        assert_eq!(
            vec![-40, 80, 160, 120, -312],
            vec![-20, 40, 80, 60, -156].volume(2.0)
        );
        assert_eq!(
            vec![-10, 20, 40, 30, -78],
            vec![-20, 40, 80, 60, -156].volume(0.5)
        );
    }

    #[test]
    fn pitch_with_time_stretch_works() {
        assert_eq!(
            vec![-10, 20, 40, 30, -78],
            vec![-10, 20, 40, 30, -78].pitch_with_time_stretch(1.0, true),
        );
        assert_eq!(
            vec![-10, 5, 20, 30, 40, 35, 30, -24, -78, -78],
            vec![-10, 20, 40, 30, -78].pitch_with_time_stretch(2.0, true),
        );
        assert_eq!(
            vec![-10, 40, -78],
            vec![-10, 20, 40, 30, -78].pitch_with_time_stretch(0.5, true),
        );
    }
}
