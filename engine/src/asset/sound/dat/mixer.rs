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
    fn pitch(self, note: u8) -> Samples;
    fn volume(self, volume: f32) -> Samples;

    fn to_wave(self) -> Vec<u8>;
}

impl SoundEffect for Samples {
    fn pitch(self, note: u8) -> Samples {
        todo!("(nenikitov): Figure out how this work")
    }

    fn volume(self, volume: f32) -> Samples {
        self.into_iter()
            .map(|s| (s as f32 * volume) as i16)
            .collect()
    }

    fn to_wave(self) -> Vec<u8> {
        const SAMPLE_RATE: usize = 16000;
        const CHANNEL_COUNT: usize = 1;
        const BITS_PER_SAMPLE: usize = 16;
        const BYTES_PER_SAMPLE: usize = BITS_PER_SAMPLE / 8;

        let size = self.len() * BYTES_PER_SAMPLE;

        "RIFF"
            .bytes()
            .chain(u32::to_be_bytes((36 + size) as u32))
            .chain("WAVE".bytes())
            .chain("fmt ".bytes())
            .chain(u32::to_le_bytes(16))
            .chain(u16::to_le_bytes(1))
            .chain(u16::to_le_bytes(CHANNEL_COUNT as u16))
            .chain(u32::to_le_bytes(SAMPLE_RATE as u32))
            .chain(u32::to_le_bytes(
                (SAMPLE_RATE * CHANNEL_COUNT * BYTES_PER_SAMPLE) as u32,
            ))
            .chain(u16::to_le_bytes((CHANNEL_COUNT * BYTES_PER_SAMPLE) as u16))
            .chain(u16::to_le_bytes(BITS_PER_SAMPLE as u16))
            .chain("data".bytes())
            .chain(u32::to_le_bytes(size as u32))
            .chain(self.into_iter().flat_map(i16::to_le_bytes))
            .collect()
    }
}

fn note_frequency(note: u8) {}
