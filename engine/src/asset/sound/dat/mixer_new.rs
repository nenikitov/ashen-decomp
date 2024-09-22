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
        if value < 0.0 {
            -(value * i16::MIN as f32) as i16
        } else {
            (value * i16::MAX as f32) as i16
        }
    }

    fn into_bytes(&self) -> Self::Bytes {
        self.to_le_bytes()
    }
}

struct Player {
    loop_count: usize,
}

impl Player {
    fn new() -> Self {
        Player { loop_count: 0 }
    }

    fn generate_sample<T: AudioSamplePoint>(&mut self, sample_rate: usize) -> T {
        self.loop_count += 1;
        todo!()
    }
}

fn main() {
    let mut player = Player::new();

    let samples: Vec<_> = std::iter::from_fn(|| {
        (player.loop_count == 0).then(|| player.generate_sample::<i16>(16000))
    })
    .collect();

    println!("Generated {} samples", samples.len());
}
