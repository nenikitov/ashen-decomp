use std::io::Cursor;

use lewton::inside_ogg::OggStreamReader;

pub mod asset_header;
pub mod chunk_header;
pub mod finetune;
pub mod mixer;
pub mod pattern_effect;
pub mod pattern_event;
pub mod t_effect;
mod t_instrument;
pub mod t_song;

// TODO(nenikitov): Make this falliable.
fn uncompress(bytes: &[u8]) -> Vec<i16> {
    if let [b'V', b'B', u1, u2, u3, c1, c2, c3, bytes @ ..] = bytes {
        let size_uncompressed = u32::from_le_bytes([*u1, *u2, *u3, 0]);
        let size_compressed = u32::from_le_bytes([*c1, *c2, *c3, 0]);

        let bytes = &bytes[..size_compressed as usize];

        let mut data =
            OggStreamReader::new(Cursor::new(bytes)).expect("Sample should be a valid OGG stream");
        let mut samples: Vec<_> = Vec::with_capacity(size_uncompressed as usize / 2);

        // TODO(nenikitov): For whatever reason, last packet seems to be wrong. We shouldn't just ignore it.
        while let Ok(Some(packet)) = data.read_dec_packet_itl() {
            samples.extend(packet);
        }

        // For whatever reason, the game has the samples stored as 16-bit signed PCM,
        // but resamples them to 8-bit PCM before playback.
        // Which would reduce the quality of music and add unnecessary code here...
        // It's 2023 and we can afford to play 16-bit PCM at 16000 Hz.

        // TODO(nenikitov): Re-add this check
        //assert_eq!(samples.len() * 2, size_uncompressed as usize);

        samples
    } else {
        // Non-compressed steam is in 8-bit PCM.
        // Because compressed data is 16-bit PCM, to keep higher quality,
        // we need to resample non-compressed stream into 16-bit.
        bytes
            .iter()
            .copied()
            .map(|sample| i8::from_le_bytes([sample]) as i16)
            .map(|sample| sample * (i16::MIN / i8::MIN as i16))
            .collect()
    }
}

fn convert_volume(volume: u8) -> f32 {
    volume as f32 / 64.
}
