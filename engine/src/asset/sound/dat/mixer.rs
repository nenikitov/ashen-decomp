use std::{collections::HashMap, rc::Rc};

use itertools::Itertools;

use super::{pattern_effect::*, pattern_event::*, t_instrument::*, t_song::*};
use crate::asset::sound::{
    dat::finetune::FineTune,
    sample::{Interpolation, Sample, SampleDataProcessing, SamplePointProcessing},
};

pub trait TSongMixer {
    fn mix(&self, restart: bool) -> Sample<i16, 1>;
}

impl TSongMixer for TSong {
    fn mix(&self, restart: bool) -> Sample<i16, 1> {
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
    const SAMPLE_RATE: usize = 16_000;
    const VOLUME_SCALE: f32 = 1.0;

    fn mix(&self, start: usize) -> Sample<i16, 1>;

    fn seconds_per_row(bpm: usize, speed: usize) -> f32;
}

impl TSongMixerUtils for TSong {
    fn mix(&self, start: usize) -> Sample<i16, 1> {
        let mut song = Sample::mono(Self::SAMPLE_RATE);

        let mut channels: Vec<_> = (0..self.patterns[0][0].len())
            .map(|_| Channel::default())
            .collect();

        let mut offset = 0;
        let mut sample_length_fractional = 0.0;
        let mut bpm = self.bpm as usize;
        let mut speed = self.speed as usize;

        // TODO!(nenikitov): Remove all `enumerate`
        for (p, pattern) in self.orders[start..].iter().enumerate() {
            for (r, row) in pattern.iter().enumerate() {
                // Update channels
                for (c, (event, channel)) in
                    Iterator::zip(row.iter(), channels.iter_mut()).enumerate()
                {
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

                    let effects: Vec<_> = event
                        .effects
                        .iter()
                        .enumerate()
                        .filter_map(|(i, effect)| {
                            if let Some(effect) = effect {
                                channel.change_effect(i, *effect);
                                Some(i)
                            } else {
                                None
                            }
                        })
                        .collect();

                    // Init effects
                    // Efffects from now on have their memory initialized
                    for effect in effects.into_iter().map(|e| {
                        channel.effects[e].expect("effect is initialized after assignment")
                    }) {
                        match effect {
                            PatternEffect::Dummy(_) => {}
                            PatternEffect::Speed(Speed::Bpm(s)) => {
                                bpm = s;
                            }
                            PatternEffect::Speed(Speed::TicksPerRow(s)) => {
                                speed = s;
                            }
                            PatternEffect::Volume(Volume::Set(volume)) => {
                                channel.volume = volume;
                            }
                            PatternEffect::Porta(Porta::Bump {
                                up: _,
                                small: _,
                                finetune: Some(finetune),
                            }) => {
                                if let Some(note) = &mut channel.note {
                                    note.finetune += finetune;
                                }
                            }
                            PatternEffect::SampleOffset(Some(offset)) => {
                                channel.sample_position = offset;
                            }
                            PatternEffect::PlaybackDirection(PlaybackDirection::Forwards) => {
                                channel.playback_direction = PlaybackDirection::Forwards
                            }
                            PatternEffect::PlaybackDirection(PlaybackDirection::Backwards) => {
                                channel.playback_direction = PlaybackDirection::Backwards;
                                if channel.sample_position == 0
                                    && let Some((_, _, sample)) =
                                        channel.get_note_instrument_sample()
                                {
                                    channel.sample_position = sample.sample_beginning().len()
                                        + sample.sample_loop().len();
                                }
                            }
                            PatternEffect::Volume(Volume::Slide(None))
                            | PatternEffect::SampleOffset(None) => {
                                unreachable!("effect memory should already be initialized")
                            }
                            _ => {}
                        };
                    }

                    // Process repeatable effects
                    for effect in channel.effects.iter().flatten() {
                        match effect {
                            PatternEffect::Volume(Volume::Slide(Some(volume))) => {
                                channel.volume += volume;
                            }
                            PatternEffect::Porta(Porta::Slide {
                                up: _,
                                finetune: Some(finetune),
                            }) => {
                                if let Some(note) = &mut channel.note {
                                    note.finetune += *finetune;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Mix current row
                let sample_length = Self::seconds_per_row(bpm, speed) * Self::SAMPLE_RATE as f32
                    + sample_length_fractional;
                sample_length_fractional = sample_length - sample_length.floor();
                let sample_length = sample_length as usize;
                let tick_length = sample_length / speed;

                let volumes = channels
                    .iter()
                    .map(|c| format!("{: >10}", c.volume))
                    .join(" ");

                for (i, c) in channels.iter_mut().enumerate() {
                    for j in 0..speed {
                        let offset = offset + j * tick_length;
                        let tick_length = if (j + 1) != speed {
                            tick_length
                        } else {
                            sample_length - j * tick_length
                        };

                        let data = c.tick(tick_length, Self::VOLUME_SCALE);
                        song.data.add_sample(&data, offset);
                    }
                }

                // Advance to next tick
                offset += sample_length;
            }
        }

        song
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
    note: Option<ChannelNote>,
    effects: [Option<PatternEffect>; 2],
    effects_memory: HashMap<PatternEffectMemoryKey, PatternEffect>,

    sample_position: usize,

    volume: f32,
    volume_envelope_position: usize,

    playback_direction: PlaybackDirection,
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
        self.playback_direction = PlaybackDirection::Forwards;
    }

    fn change_volume(&mut self, volume: PatternEventVolume) {
        self.volume = match volume {
            PatternEventVolume::Sample => {
                if let Some((_, _, sample)) = self.get_note_instrument_sample() {
                    sample.volume
                } else {
                    0.0
                }
            }

            PatternEventVolume::Value(value) => value,
        };
    }

    fn get_note_instrument_sample(&self) -> Option<(&ChannelNote, &Rc<TInstrument>, &Rc<TSample>)> {
        if let Some(note) = &self.note
            && let Some(PatternEventInstrumentKind::Predefined(instrument)) = self.instrument
            && let TInstrumentSampleKind::Predefined(sample) =
                &instrument.samples[note.finetune.note().clamp(0, 95) as usize]
        {
            Some((note, instrument, sample))
        } else {
            None
        }
    }

    fn tick(&mut self, duration: usize, volume_scale: f32) -> Vec<[i16; 1]> {
        if let Some((note, instrument, sample)) = self.get_note_instrument_sample() {
            // Generate data
            let volume_envelope = match &instrument.volume {
                TInstrumentVolume::Envelope(envelope) => {
                    let (envelope, default) = if note.on {
                        (envelope.volume_beginning(), envelope.volume_loop())
                    } else {
                        (envelope.volume_end(), 0.0)
                    };
                    envelope
                        .get(self.volume_envelope_position)
                        .map(ToOwned::to_owned)
                        .unwrap_or(default)
                }
                TInstrumentVolume::Constant(volume) => *volume,
            };
            let volume = volume_scale * volume_envelope * self.volume.clamp(0.0, 4.0);

            let pitch_factor = (note.finetune + sample.finetune).pitch_factor();
            let duration_scaled = (duration as f64 / pitch_factor).round() as usize;
            let mut sample = match self.playback_direction {
                PlaybackDirection::Forwards => sample
                    .sample_beginning()
                    .iter()
                    .chain(sample.sample_loop().iter().cycle())
                    .skip(self.sample_position)
                    .take(duration_scaled + 1)
                    .copied()
                    .collect::<Vec<_>>(),
                PlaybackDirection::Backwards => sample
                    .sample_beginning()
                    .iter()
                    .chain(sample.sample_loop())
                    .rev()
                    .skip(self.sample_position)
                    .take(duration_scaled + 1)
                    .copied()
                    .collect::<Vec<_>>(),
            };
            let first_sample_after = sample.pop();
            let pitch_factor = duration as f32 / sample.len() as f32;

            let mut data = sample.volume(volume).stretch(
                pitch_factor,
                Interpolation::Linear {
                    first_sample_after: first_sample_after.map(|s| s.volume(volume)),
                },
            );

            // Update
            self.sample_position += duration_scaled;
            self.volume_envelope_position += 1;

            // Return
            data
        } else {
            vec![]
        }
    }

    pub fn recall_effect_with_memory(&mut self, effect: PatternEffect) -> PatternEffect {
        if let Some(key) = effect.memory_key() {
            if !effect.is_empty() {
                self.effects_memory.insert(key, effect);
            }

            self.effects_memory[&key]
        } else {
            effect
        }
    }

    fn change_effect(&mut self, i: usize, effect: PatternEffect) {
        self.effects[i] = Some(self.recall_effect_with_memory(effect));
    }
}
