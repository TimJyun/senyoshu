// use std::collections::hash_map::RandomState;
// use std::hash::{BuildHasher, Hasher};
//
// use crate::types::word::mean_entry::MeanEntry;
// use crate::types::word::word::Word;
//
// use crate::types::word::word_entry::WordEntry;
//
// pub struct WordGenerator;
//
// impl WordGenerator {
//     pub fn generate(seed: u64, num: u64) -> WordEntry {
//         let s = RandomState::new();
//         let mut hasher = s.build_hasher();
//         hasher.write_u64(seed);
//
//         let hana = hasher.finish();
//
//         let word = Word::from_u64(num);
//
//         let word_define = WordDefine {
//             tones: Vec::new(),
//
//             // Tones([
//             //     hana & (1 << 0) == 1,
//             //     hana & (1 << 1) == 1,
//             //     hana & (1 << 2) == 1,
//             //     hana & (1 << 3) == 1,
//             //     hana & (1 << 4) == 1,
//             //     hana & (1 << 5) == 1,
//             // ]),
//             means: vec![],
//             detailed: None,
//         };
//         hasher.write_u64(hana);
//         let hana = hasher.finish();
//
//         let mut means = Vec::with_capacity((hana % 11) as usize);
//
//         for i in 0..(hana % 11) {
//             let mean = MeanEntry {
//                 parts_of_speech: Default::default(),
//                 explanation: Default::default(),
//                 example_sentences: vec![],
//             };
//             means.push(mean)
//         }
//         WordEntry {
//             word,
//             word_define: Default::default(),
//         }
//     }
// }
