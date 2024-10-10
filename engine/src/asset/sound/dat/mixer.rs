use std::{
    collections::HashMap,
    ops::{Add, Sub},
    rc::Rc,
    time::Duration,
};

use itertools::Itertools;

use super::{
    finetune::FineTune, pattern_effect::*, pattern_event::*, t_instrument::*, t_song::TSong,
};
use crate::asset::sound::sample::{AudioBuffer, AudioSamplePoint};

#[derive(Default, Clone, Debug)]
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
    volume_target: f32,
    volume_actual: f32,
    effects: [Option<PatternEffect>; 2],
    effects_memory: HashMap<PatternEffectMemoryKey, PatternEffect>,
    note_delay: usize,
    retrigger: bool,

    previous: Option<(Box<Self>, f64)>,

    pos_sample: f64,
    pos_volume_envelope: usize,
    direction: PlaybackDirection,
}

impl PlayerChannel {
    // Length of a transition between the current and the next sample in seconds
    // Too large of a time and samples will audibly blend and play 2 notes at the same time, which sounds weird.
    // Too little and transitions between notes will click.
    // Chosen value is a bit of an arbitrary value that I found sounds nice.
    // It amounts to:
    // - 13 samples at 16000
    // - 35 samples at 44100
    // - 38 samples at 48000
    const SAMPLE_BLEND: f64 = Duration::from_micros(800).as_secs_f64();
    // Maximum difference in volume between 2 audio samples
    // Volume as in channels volume, does not account for samples
    // A bit of an arbitrary amount too
    const MAX_VOLUME_CHANGE: f32 = 1. / 128.;

    fn note_cut(&mut self) {
        self.volume = 0.;
        self.volume_actual = 0.;
    }

    fn pos_reset(&mut self) {
        self.pos_sample = 0.;
        self.pos_volume_envelope = 0;
        self.direction = PlaybackDirection::Forwards;
    }

    fn compute_volume_envelope(&self) -> Option<f32> {
        self.instrument.as_ref().and_then(|i| match &i.volume {
            TInstrumentVolume::Envelope(envelope) => {
                if self.note.on {
                    Some(
                        envelope
                            .volume_beginning()
                            .get(self.pos_volume_envelope)
                            .copied()
                            .unwrap_or(envelope.volume_loop()),
                    )
                } else {
                    envelope
                        .volume_end()
                        .get(
                            self.pos_volume_envelope
                                .saturating_sub(envelope.volume_beginning().len()),
                        )
                        .copied()
                }
            }
            TInstrumentVolume::Constant(_) => Some(1.),
        })
    }

    fn generate_current_sample(&mut self, step: f64) -> f32 {
        if let Some(instrument) = &self.instrument
            && let Some(sample) = &self.sample
            && let Some(note) = self.note.finetune
            && self.note_delay == 0
            && let Some(volume_envelope) = self.compute_volume_envelope()
            && let Some(value) = sample.get(self.pos_sample)
        {
            let pitch_factor = (note + sample.finetune).pitch_factor();
            let step = step / pitch_factor;
            self.pos_sample += match self.direction {
                PlaybackDirection::Forwards => step,
                PlaybackDirection::Backwards => -step,
            };

            self.volume_target = volume_envelope * self.volume;
            self.volume_actual = advance_to(
                self.volume_actual,
                self.volume_target,
                Self::MAX_VOLUME_CHANGE,
            );

            value.into_normalized_f32() * self.volume_actual
        } else {
            0.
        }
    }

    fn generate_sample(&mut self, step: f64) -> f32 {
        let current_sample = self.generate_current_sample(step);

        if let Some((previous, position)) = &mut self.previous {
            let factor = (*position / Self::SAMPLE_BLEND) as f32;
            let previous_sample = previous.generate_sample(step);

            *position += step;
            if *position >= Self::SAMPLE_BLEND {
                self.previous = None;
            }

            previous_sample + factor * (current_sample - previous_sample)
        } else {
            current_sample
        }
    }

    fn trigger_note(&mut self) {
        // Previous state is kept to subtly blend in notes to remove clicks.

        // Disregard previous state before `self.clone` so we don't have a fully recursive structure.
        self.previous = None;
        self.previous = Some((Box::new(self.clone()), 0.));

        self.pos_reset();
    }

    fn change_instrument(&mut self, instrument: PatternEventInstrument) {
        if let PatternEventInstrument::Instrument(instrument) = instrument {
            self.instrument = Some(instrument);
        }

        if self.instrument.is_some() {
            self.trigger_note();
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
                    self.sample
                        .clone_from(&instrument.samples[note.note() as usize]);
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
            && (!self.note.on
                || envelope
                    .sustain
                    .map_or(true, |s| self.pos_volume_envelope < s))
        {
            self.pos_volume_envelope += 1;
        }
    }

    fn clear_effects(&mut self) {
        self.effects = [None, None];
    }
}

trait MinMax {
    fn generic_min(a: Self, b: Self) -> Self;
    fn generic_max(a: Self, b: Self) -> Self;
}

macro_rules! impl_min_max {
    ($($ty:tt),*) => {
        $(impl MinMax for $ty {
            fn generic_min(a: Self, b: Self) -> Self {
                $ty::min(a, b)
            }

            fn generic_max(a: Self, b: Self) -> Self {
                $ty::max(a, b)
            }
        })*
    };
}

impl_min_max!(f32, FineTune);

fn advance_to<T>(from: T, to: T, step: T) -> T
where
    T: PartialOrd + Add<Output = T> + Sub<Output = T> + MinMax,
{
    use std::cmp::Ordering;
    match from.partial_cmp(&to) {
        Some(Ordering::Less) => T::generic_min(from + step, to),
        Some(Ordering::Greater) => T::generic_max(from - step, to),
        Some(Ordering::Equal) | None => from,
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
    jump: Option<usize>,

    tempo: usize,
    bpm: usize,
    volume_global_target: f32,
    volume_global_actual: f32,
    volume_amplification: f32,

    channels: Vec<PlayerChannel>,
}

impl<'a> Player<'a> {
    fn new(song: &'a TSong, sample_rate: usize, amplification: f32) -> Self {
        Self {
            song,
            sample_rate,
            time_in_tick: 0.,
            pos_loop: 0,
            pos_pattern: 0,
            pos_row: 0,
            pos_tick: 0,
            jump: None,
            tempo: song.speed as usize,
            bpm: song.bpm as usize,
            volume_global_target: 1.,
            volume_global_actual: 0.,
            volume_amplification: amplification,
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
            .map(|c| c.generate_sample(step))
            .sum::<f32>();
        self.volume_global_actual = advance_to(
            self.volume_global_actual,
            self.volume_global_target,
            PlayerChannel::MAX_VOLUME_CHANGE,
        );
        S::from_normalized_f32(sample * self.volume_global_actual * self.volume_amplification)
    }

    fn tick(&mut self) {
        if self.pos_tick == 0 {
            self.row();
        }

        for channel in &mut self.channels {
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
                            channel.note.finetune = channel
                                .note
                                .finetune
                                .map(|finetune| advance_to(finetune, finetune_initial, step));
                        }
                    }
                    E::Porta(Porta::Slide {
                        finetune: Some(finetune),
                        ..
                    }) => {
                        channel.note.finetune = channel.note.finetune.map(|f| (f + finetune));
                    }
                    E::NoteDelay(_) => {
                        channel.note_delay = channel.note_delay.saturating_sub(1);
                    }
                    E::RetriggerNote(frequency) => {
                        if frequency != 0 && self.pos_tick != 0 && (self.pos_tick % frequency) == 0
                        {
                            // HACK(nenikitov): After processing all effects, retrigger will happen by callign `trigger_note` and `advance_envelopes`
                            // Because of mutability here
                            channel.retrigger = true;
                        }
                    }
                    // Noops - no tick
                    E::Speed(..)
                    | E::PatternBreak
                    | E::PatternJump(..)
                    | E::Volume(Volume::Set(..) | Volume::Bump { .. })
                    | E::Porta(Porta::Tone(..) | Porta::Bump { .. })
                    | E::GlobalVolume(..)
                    | E::SampleOffset(..)
                    | E::PlaybackDirection(..) => {}
                    // Unreachable because memory has to be initialized
                    E::Volume(Volume::Slide(None))
                    | E::Porta(Porta::Slide { finetune: None, .. }) => {
                        unreachable!("Effects should have their memory initialized")
                    }
                }
            }

            // TODO(nenikitov): Move this to the event `matchc
            if channel.retrigger {
                channel.trigger_note();
                channel.advance_envelopes();
                channel.retrigger = false;
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
        }
        if self.pos_pattern >= self.song.orders.len() {
            self.pos_pattern = self.song.restart_order as usize;
            self.pos_loop += 1;
        }

        self.time_in_tick += 2.5 / (self.bpm as f64);
    }

    fn row(&mut self) {
        if let Some(pos_pattern) = self.jump.take() {
            if pos_pattern <= self.pos_pattern {
                self.pos_loop += 1;
            }
            self.pos_pattern = pos_pattern;
            self.pos_row = 0;
        }

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

            if let Some(note) = event.note {
                channel.change_note(note);
            }

            if let Some(volume) = event.volume {
                channel.change_volume(volume);
            }

            channel.clear_effects();
            for (i, effect) in event
                .effects
                .iter()
                .enumerate()
                .filter_map(|(i, e)| e.map(|e| (i, e)))
            {
                use PatternEffect as E;

                channel.change_effect(i, effect);
                match channel.effects[i].expect("`change_effect` sets the effect") {
                    // Init effects
                    E::Speed(Speed::Bpm(bpm)) => {
                        self.bpm = bpm;
                    }
                    E::Speed(Speed::TicksPerRow(ticks_per_row)) => {
                        self.tempo = ticks_per_row;
                    }
                    E::PatternBreak => {
                        self.jump = Some(self.pos_pattern + 1);
                    }
                    E::PatternJump(pos_pattern) => {
                        self.jump = Some(pos_pattern);
                    }
                    E::GlobalVolume(volume) => {
                        self.volume_global_target = volume;
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
                            channel.pos_sample = sample
                                .buffer
                                .index_to_seconds(sample.buffer.len_samples().saturating_sub(1));
                        }
                    }
                    E::SampleOffset(Some(offset)) => {
                        channel.pos_sample = offset as f64 / TSample::SAMPLE_RATE as f64;
                    }
                    E::NoteDelay(delay) => {
                        channel.note_delay = delay;
                    }
                    // Noops - no init
                    E::Volume(Volume::Slide(..))
                    | E::Porta(Porta::Tone(..) | Porta::Slide { .. })
                    | E::RetriggerNote(..) => {}
                    // Unreachable because memory has to be initialized
                    E::Volume(Volume::Bump { volume: None, .. })
                    | E::Porta(Porta::Tone(None) | Porta::Bump { finetune: None, .. })
                    | E::SampleOffset(None) => {
                        unreachable!("Effects should have their memory initialized")
                    }
                }
            }
        }
    }
}

pub trait TSongMixer {
    fn mix(&self) -> AudioBuffer<i16>;
}

impl TSongMixer for TSong {
    fn mix(&self) -> AudioBuffer<i16> {
        const SAMPLE_RATE: usize = 48000;
        const AMPLIFICATION: f32 = 0.375;

        let mut player = Player::new(self, SAMPLE_RATE, AMPLIFICATION);

        let samples: Vec<_> = std::iter::from_fn(|| {
            if player.pos_loop == 0 {
                Some(player.generate_sample::<i16>())
            } else {
                None
            }
        })
        .collect();

        AudioBuffer {
            data: samples,
            sample_rate: player.sample_rate,
        }
    }
}
