use std::fmt::Display;

use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default,Hash)]
pub struct Tones(pub [bool; 6]);

pub static TONE_SIGNS: [char; 6] = ['⓪', '①', '②', '③', '④', '⑤'];

// #[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub enum NumTone {
//     Zero = 0,
//     One = 1,
//     Two = 2,
//     Three = 3,
//     Four = 4,
//     Five = 5,
// }


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Tone(usize);


// impl Into<char> for NumTone {
//     fn into(self) -> char {
//         match self {
//             NumTone::Zero => '⓪',
//             NumTone::One => '①',
//             NumTone::Two => '②',
//             NumTone::Three => '③',
//             NumTone::Four => '④',
//             NumTone::Five => '⑤',
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
// pub enum Tone {
//     NumTone(NumTone),
//     ComplexTone(ComplexTone),
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
// pub struct SoundMeta {
//     ruby: String,
//     tone: Tone,
//     speaker: String,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
// struct ComplexTone {
//     start_high: bool,
//     sections: Vec<usize>,
// }

impl Display for Tones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rv = self
            .0
            .iter()
            .zip(TONE_SIGNS)
            .filter(|it| *it.0)
            .map(|it| it.1)
            .collect::<String>();

        write!(f,"{rv}")
    }
}

impl Tones {
    pub fn is_undefined(&self) -> bool {
        self.0.iter().all(|it| !*it)
    }

    pub fn iter(&self) -> impl Iterator<Item=Tone> {
        self.0
            .into_iter()
            .enumerate()
            .filter(|(_, it)| *it)
            .map(|(idx, _)| Tone(idx))
    }
}
