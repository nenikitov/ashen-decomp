use itertools::Itertools;

type Sample = i16;
type Samples = Vec<Sample>;

pub struct Mixer {
    samples: Samples,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn add_samples(&mut self, sample: &Samples, offset: usize) {
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

    pub fn mix(self) -> Samples {
        self.samples
    }
}

pub trait SoundEffect {
    fn pitch(&self, note: u8) -> Samples;
    fn volume(&self, volume: f32) -> Samples;
}

impl SoundEffect for Samples {
    fn pitch(&self, note: u8) -> Samples {
        todo!()
    }

    fn volume(&self, volume: f32) -> Samples {
        self.iter().map(|&s| (s as f32 * volume) as i16).collect()
    }
}

fn note_frequency(note: u8) {}
