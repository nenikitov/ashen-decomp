pub mod nom;

pub mod fs {
    /// Gets a path relative to the workspace directory.
    macro_rules! workspace_file {
        ($file:literal) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../", $file)
        };
    }

    /// Writes to a file creating the directory automatically.
    pub fn output_file<P, C>(path: P, contents: C) -> io::Result<()>
    where
        P: AsRef<Path>,
        C: AsRef<[u8]>,
    {
        fn inner(path: &Path, contents: &[u8]) -> io::Result<()> {
            let parent = path.parent().ok_or(io::ErrorKind::InvalidFilename)?;
            fs::create_dir_all(parent)?;
            fs::write(path, contents)
        }

        inner(path.as_ref(), contents.as_ref())
    }

    use std::{fs, io, path::Path};

    pub(crate) use workspace_file;
}

pub mod format {
    use crate::asset::color_map::Color;

    pub trait PpmFile {
        fn to_ppm(&self) -> Vec<u8>;
    }

    impl PpmFile for Vec<Vec<Color>> {
        fn to_ppm(&self) -> Vec<u8> {
            let width = self[0].len();
            let height = self.len();

            format!("P6 {} {} 255\n", width, height)
                .bytes()
                .chain(
                    self.iter()
                        .flat_map(|r| r.iter().flat_map(|c| [c.r, c.g, c.b]).collect::<Vec<_>>()),
                )
                .collect()
        }
    }

    pub trait WaveFile {
        fn to_wave(&self, sample_rate: usize, channel_count: usize) -> Vec<u8>;
    }

    impl WaveFile for Vec<i16> {
        fn to_wave(&self, sample_rate: usize, channel_count: usize) -> Vec<u8> {
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
                .chain(u16::to_le_bytes(channel_count as u16))
                .chain(u32::to_le_bytes(sample_rate as u32))
                .chain(u32::to_le_bytes(
                    (sample_rate * channel_count * BYTES_PER_SAMPLE) as u32,
                ))
                .chain(u16::to_le_bytes((channel_count * BYTES_PER_SAMPLE) as u16))
                .chain(u16::to_le_bytes(BITS_PER_SAMPLE as u16))
                .chain("data".bytes())
                .chain(u32::to_le_bytes(size as u32))
                .chain(self.iter().flat_map(|s| s.to_le_bytes()))
                .collect()
        }
    }
}
