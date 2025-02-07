use std::iter;

use itertools::Either;

// https://www.amazingtalker.cn/blog/zh-tw/zh-jap/11497/
// https://jp.hjenglish.com/new/p384457/
// https://zh.wikibooks.org/zh-hk/%E6%97%A5%E8%AF%AD/%E5%85%A5%E9%96%80%E8%AA%B2%E7%A8%8B/%E6%95%B8%E5%AD%97
use crate::types::word::word::{Word, WordElement};
use crate::util::iter_util::WithNextMutMapItertool;

// 0 = ゼロ/まる
// 4 = よん
// 7 = なな
const UNITS_KANJI: [char; 11] = [
    '零', '一', '二', '三', '四', '五', '六', '七', '八', '九', TEN_KANJI,
];
const UNITS_KANA: [&str; 11] = [
    "れい",
    "いち",
    "に",
    "さん",
    "し",
    "ご",
    "ろく",
    "しち",
    "はち",
    "きゅう",
    TEN_KANA,
];

const TEN_KANJI: char = '十';
const TEN_KANA: &str = "じゅう";

const HUNDRED_KANJI: char = '百';
const HUNDRED_KANA: &str = "ひゃく";

const THOUSAND_KANJI: char = '千';
const THOUSAND_KANA: &str = "せん";

const PACKS_KANJI: [char; 5] = ['京', '兆', '億', '万', ' '];
const PACKS_KANA: [&str; 5] = ["けい", "ちょう", "おく", "まん", ""];

const DOT: &str = "てん";

fn less_than_10(num: u64) -> impl Iterator<Item=WordElement> {
    iter::once(WordElement {
        txt: UNITS_KANJI[num as usize].to_string(),
        ruby: UNITS_KANA[num as usize].to_string(),
        proto:String::new()
    })
}

fn more_than_9_less_than_100(num: u64) -> impl Iterator<Item=WordElement> {
    [
        WordElement {
            txt: UNITS_KANJI[(num / 10) as usize].to_string(),
            ruby: UNITS_KANA[(num / 10) as usize].to_string(),
        proto:String::new()
        },
        WordElement {
            txt: TEN_KANJI.to_string(),
            ruby: TEN_KANA.to_string(),
        proto:String::new()
        },
    ]
        .into_iter()
        .chain(less_than_10(num % 10))
}

fn more_than_99_less_than_1000(num: u64) -> impl Iterator<Item=WordElement> {
    [
        WordElement {
            txt: UNITS_KANJI[(num / 100) as usize].to_string(),
            ruby: UNITS_KANA[(num / 100) as usize].to_string(),
        proto:String::new()
        },
        WordElement {
            txt: HUNDRED_KANJI.to_string(),
            ruby: HUNDRED_KANA.to_string(),
        proto:String::new()
        },
    ]
        .into_iter()
        .chain(more_than_9_less_than_100(num % 100))
}

fn more_than_999_less_than_10000(num: u64) -> impl Iterator<Item=WordElement> {
    [
        WordElement {
            txt: UNITS_KANJI[(num / 1000) as usize].to_string(),
            ruby: UNITS_KANA[(num / 1000) as usize].to_string(),
        proto:String::new()
        },
        WordElement {
            txt: THOUSAND_KANJI.to_string(),
            ruby: THOUSAND_KANA.to_string(),
        proto:String::new()
        },
    ]
        .into_iter()
        .chain(more_than_99_less_than_1000(num % 1000))
}

fn less_than_100(num: u64) -> impl Iterator<Item=WordElement> {
    if num < 10 {
        Either::Left(less_than_10(num))
    } else {
        Either::Right(more_than_9_less_than_100(num))
    }
}

fn less_than_1000(num: u64) -> impl Iterator<Item=WordElement> {
    if num < 100 {
        Either::Left(less_than_100(num))
    } else {
        Either::Right(more_than_99_less_than_1000(num))
    }
}

fn less_than_10000(num: u64) -> impl Iterator<Item=WordElement> {
    if num < 1000 {
        Either::Left(less_than_1000(num))
    } else {
        Either::Right(more_than_999_less_than_10000(num))
    }
}

pub(crate) fn number_to_japanese(num: u64) -> Word {
    let rv = [
        num / (10u64.pow(16)),
        (num / (10u64.pow(12))) % (10u64.pow(4)),
        (num / (10u64.pow(8))) % (10u64.pow(4)),
        (num / (10u64.pow(4))) % (10u64.pow(4)),
        (num / (10u64.pow(0))) % (10u64.pow(4)),
    ]
        .into_iter()
        .enumerate()
        .filter(|it| it.1 > 0)
        .map(|(i, num)| {
            less_than_10000(num).chain(iter::once(WordElement {
                txt: PACKS_KANJI[i].to_string(),
                ruby: PACKS_KANA[i].to_string(),
                proto:String::new()
            }))
        })
        .flatten()
        .with_next_mut_map(|mut it, next| {
            if let Some(next) = next {
                match (it.txt.as_str(), next.txt.as_str()) {
                    ("一", "十") => {
                        it.ruby = String::new();
                    }
                    ("一", "百") => {
                        it.ruby = String::new();
                    }
                    ("一", "千") => {
                        it.ruby = String::new();
                    }
                    ("零", "零") => {
                        next.ruby = String::new();
                    }
                    ("十", "零") => {
                        next.ruby = String::new();
                    }
                    ("百", "零") => {
                        next.ruby = String::new();
                    }
                    ("千", "零") => {
                        next.ruby = String::new();
                    }
                    ("三", "百") => {
                        next.ruby = String::from("びゃく");
                    }
                    ("三", "千") => {
                        next.ruby = String::from("ぜん");
                    }
                    ("六", "百") => {
                        it.ruby = String::from("ろっ");
                        next.ruby = String::from("ぴゃく");
                    }
                    ("八", "百") => {
                        it.ruby = String::from("はっ");
                        next.ruby = String::from("ぴゃく");
                    }
                    ("八", "千") => {
                        it.ruby = String::from("はっ");
                    }
                    ("一", "兆") => {
                        it.ruby = String::from("いっ");
                    }
                    _ => {}
                }
            }
            it
        })
        .filter(|it| !it.ruby.is_empty())
        .collect::<Vec<_>>();

    if rv.is_empty() {
        Word {
            elements: Vec::from([WordElement {
                txt: "零".to_string(),
                ruby: "れい".to_string(),
                proto:String::new()
            }]),
            tones: Default::default(),
        }
    } else {
        Word { elements: rv, tones: Default::default() }
    }
}
