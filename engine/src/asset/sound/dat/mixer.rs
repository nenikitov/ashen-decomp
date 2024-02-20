use super::{pattern_event::*, t_instrument::*, t_song::*};
use crate::asset::sound::dat::finetune::FineTune;

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

    fn seconds_per_row(bpm: usize, speed: usize) -> f32;
}

impl TSongMixerUtils for TSong {
    fn mix(&self, start: usize) -> Sample {
        let mut m = Mixer::new();

        let mut channels: Vec<_> = (0..self.patterns[0][0].len())
            .map(|_| Channel::default())
            .collect();

        let mut offset = 0;
        let mut sample_length_fractional = 0.0;
        let mut bpm = self.bpm as usize;
        let mut speed = self.speed as usize;

        for pattern in &self.orders[start..] {
            for row in &**pattern {
                // Update channels
                for (c, event) in row.iter().enumerate() {
                    let channel = &mut channels[c];

                    // Process note
                    if let Some(note) = event.note {
                        channel.change_note(note);
                    }
                    if let Some(instrument) = &event.instrument {
                        channel.change_instrument(instrument);
                    }
                    if let Some(volume) = event.volume {
                        channel.change_volume(volume);
                    }

                    // Process effects
                    for effect in event.effects.iter().flatten() {
                        match effect.kind {
                            // TODO(nenikitov): Add effects
                            PatternEffectKind::Speed => {
                                if effect.value >= 0x20 {
                                    bpm = effect.value as usize;
                                } else {
                                    speed = effect.value as usize;
                                }
                            }
                            PatternEffectKind::SampleOffset => {
                                channel.sample_position = effect.value as usize * 256;
                            }
                            _ => {}
                        };
                    }
                }

                // Mix current row
                let sample_length = Self::seconds_per_row(bpm, speed) * Self::SAMPLE_RATE as f32
                    + sample_length_fractional;
                sample_length_fractional = sample_length - sample_length.floor();
                let sample_length = sample_length as usize;
                let tick_length = sample_length / speed;

                for (i, c) in channels.iter_mut().enumerate() {
                    for j in 0..speed {
                        let offset = offset + j * tick_length;
                        let tick_length = if (j + 1) != speed {
                            tick_length
                        } else {
                            sample_length - j * tick_length
                        };

                        let data = c.tick(tick_length);
                        m.add_sample(&data, offset);
                    }
                }

                // Advance to next tick
                offset += sample_length;
            }
        }

        m.mix()
    }

    fn seconds_per_row(bpm: usize, speed: usize) -> f32 {
        // TODO(nenikitov): The formula from the game is `5 / 2 / BPM * SPEED`, not sure why
        2.5 / bpm as f32 * speed as f32
    }
}

struct ChannelNote {
    finetune: FineTune,
    on: bool,
}

#[derive(Default)]
struct Channel<'a> {
    instrument: Option<&'a PatternEventInstrumentKind>,

    sample_position: usize,

    volume: PatternEventVolume,
    volume_envelope_position: usize,
    note: Option<ChannelNote>,
}

impl<'a> Channel<'a> {
    fn change_note(&mut self, note: PatternEventNote) {
        match (&mut self.note, note) {
            (None, PatternEventNote::Off) => {
                self.note = None;
            }
            (None, PatternEventNote::On(target)) => {
                self.note = Some(ChannelNote {
                    finetune: target,
                    on: true,
                });
            }
            (Some(current), PatternEventNote::Off) => {
                current.on = false;
                self.volume_envelope_position = 0;
            }
            (Some(current), PatternEventNote::On(target)) => {
                current.finetune = target;
                current.on = true;
            }
        }
    }

    fn change_instrument(&mut self, instrument: &'a PatternEventInstrumentKind) {
        self.instrument = Some(instrument);
        self.sample_position = 0;
        self.volume_envelope_position = 0;
    }

    fn change_volume(&mut self, volume: PatternEventVolume) {
        self.volume = volume;
    }

    fn tick(&mut self, duration: usize) -> Sample {
        if let Some(instrument) = self.instrument
            && let Some(note) = &self.note
            && let PatternEventInstrumentKind::Predefined(instrument) = instrument
            && let TInstrumentSampleKind::Predefined(sample) =
                &instrument.samples[note.finetune.note() as usize]
        {
            // Generate data
            let volume_note = match self.volume {
                PatternEventVolume::Sample => sample.volume,
                PatternEventVolume::Value(value) => value,
            };
            let volume_envelope = match &instrument.volume {
                TInstrumentVolume::Envelope(envelope) => {
                    if note.on {
                        envelope
                            .volume_beginning()
                            .get(self.volume_envelope_position)
                            .map(ToOwned::to_owned)
                            .unwrap_or(envelope.volume_loop())
                    } else {
                        envelope
                            .volume_end()
                            .get(self.volume_envelope_position)
                            .map(ToOwned::to_owned)
                            .unwrap_or_default()
                    }
                }
                TInstrumentVolume::Constant(volume) => *volume,
            };

            let pitch_factor = (note.finetune + sample.finetune).pitch_factor();

            let duration_scaled = (duration as f64 / pitch_factor).round() as usize;

            let mut data = sample
                .sample_beginning()
                .iter()
                .chain(sample.sample_loop().iter().cycle())
                .skip(self.sample_position + 1)
                .take(duration_scaled)
                .copied()
                .collect::<Vec<_>>();

            let pitch_factor = (duration + 1) as f32 / data.len() as f32;
            let mut data = data
                .volume(volume_note * volume_envelope)
                .pitch_with_time_stretch(pitch_factor, None);
            data.truncate(duration);

            // Update
            self.sample_position += duration_scaled;
            self.volume_envelope_position += 1;

            // Return
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
    fn pitch_with_time_stretch(self, factor: f32, next_sample: Option<i16>) -> Self;
}

impl SoundEffect for Sample {
    fn volume(self, volume: f32) -> Self {
        self.into_iter()
            .map(|s| (s as f32 * volume) as i16)
            .collect()
    }

    fn pitch_with_time_stretch(self, factor: f32, next_sample: Option<i16>) -> Self {
        // TODO(nenikitov): Look into linear interpolation
        let len = (self.len() as f32 * factor).round() as usize;

        (0..len)
            .map(|i| {
                let frac = i as f32 / factor;
                let index = frac.floor() as usize;
                let frac = frac - index as f32;

                let sample_1 = self[index] as f32;
                let sample_2 = if self.len() > index + 1 {
                    self[index + 1]
                } else if let Some(next_sample) = next_sample {
                    next_sample
                } else {
                    self[index]
                } as f32;

                ((1.0 - frac) * sample_1 + frac * sample_2).floor() as i16
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
            vec![-10, 20, 40, 30, -78].pitch_with_time_stretch(1.0, None),
        );
        assert_eq!(
            vec![-10, -10, 20, 20, 40, 40, 30, 30, -78, -78],
            vec![-10, 20, 40, 30, -78].pitch_with_time_stretch(2.0, None),
        );
        assert_eq!(
            vec![-10, 40, -78],
            vec![-10, 20, 40, 30, -78].pitch_with_time_stretch(0.5, None),
        );
    }
}
