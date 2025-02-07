use std::io::{Read, Write};

use base64::Engine;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

pub struct PermanentStorage;

impl PermanentStorage {
    pub fn get<T>(key: impl AsRef<str>) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let base64 = LocalStorage::get::<String>(key).ok()?;
        let uncompressed = base64::prelude::BASE64_STANDARD_NO_PAD
            .decode(base64)
            .ok()?;
        let mut cbor = Vec::new();
        let mut decompressor = lz4_flex::frame::FrameDecoder::new(uncompressed.as_slice());
        decompressor.read_to_end(&mut cbor).ok()?;
        drop(decompressor);

        ciborium::from_reader(cbor.as_slice()).ok()
    }

    pub fn set<T>(key: impl AsRef<str>, value: T) -> Option<()>
    where
        T: Serialize,
    {
        let mut cbor = Vec::new();
        ciborium::into_writer(&value, &mut cbor).unwrap();

        let mut compressed = Vec::new();
        let mut compressor = lz4_flex::frame::FrameEncoder::new(&mut compressed);
        compressor.write_all(cbor.as_mut_slice()).unwrap();
        compressor.finish().unwrap();

        let mut base64 = String::with_capacity((compressed.len() as f32 * 1.34f32) as usize + 2);
        base64::prelude::BASE64_STANDARD_NO_PAD.encode_string(compressed, &mut base64);

        LocalStorage::set::<String>(key, base64).unwrap();

        Some(())
    }
}
