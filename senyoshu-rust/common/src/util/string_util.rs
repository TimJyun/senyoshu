use itertools::Itertools;

use crate::util::iter_util::WithNextMutMapItertool;

pub struct StringUtil;

impl StringUtil {
    pub fn is_kanji(c: char) -> bool {
        let c = c as u32;

        //A
        (c >= 0x3400 && c <= 0x4DBF) ||
            //中日韩统一表意文字
            (c >= 0x4E00 && c <= 0x9FFF) ||
            //cjk兼容
            (c >= 0xF900 && c <= 0xFAFF) ||
            //B
            (c >= 0x20000 && c <= 0x2A6DF) ||
            //C,D,E,F,I
            (c >= 0x2A700 && c <= 0x2EE5F) ||
            //cjk兼容扩充
            (c >= 0x2F800 && c <= 0x2FA1F) ||
            //G,H
            (c >= 0x30000 && c <= 0x323AF)
    }

    pub fn ruby_to_katakana(ruby: impl AsRef<str>) -> String {
        ruby.as_ref()
            .chars()
            .map(|c| StringUtil::to_katakana(c))
            .with_next_mut_map(|it, next| {
                if let Some(next) = next {
                    //todo:小元音还不知道该怎么处理
                    match next {
                        'ア' => {
                            if KATAKANA_A.contains(it) {
                                *next = KATAKANA_MACRON;
                            }
                        }
                        'イ' => {
                            if KATAKANA_I.contains(it) || KATAKANA_E.contains(it) {
                                *next = KATAKANA_MACRON;
                            }
                        }
                        'ウ' => {
                            if KATAKANA_U.contains(it) || KATAKANA_O.contains(it) {
                                *next = KATAKANA_MACRON;
                            }
                        }
                        // *i + e 不合成长音
                        'エ' => {
                            if KATAKANA_E.contains(it) {
                                *next = KATAKANA_MACRON;
                            }
                        }
                        // *u + o 不合成长音
                        'オ' => {
                            if KATAKANA_O.contains(it) {
                                *next = KATAKANA_MACRON;
                            }
                        }
                        _ => {}
                    }
                }
                it
            })
            .collect::<String>()
    }

    pub fn is_kana(c: char) -> bool {
        StringUtil::is_hiragana(c) || StringUtil::is_katakana(c)
    }

    pub fn is_hiragana(c: char) -> bool {
        HIRAGANA.contains(c)
    }

    pub fn is_katakana(c: char) -> bool {
        KATAKANA.contains(c) || c == KATAKANA_MACRON
    }

    pub fn to_hiragana(c: char) -> char {
        if StringUtil::is_katakana(c) {
            unsafe { char::from_u32_unchecked(c as u32 + 'あ' as u32 - 'ア' as u32) }
        } else {
            c
        }
    }

    pub fn to_katakana(c: char) -> char {
        if StringUtil::is_hiragana(c) {
            unsafe { char::from_u32_unchecked(c as u32 + 'ア' as u32 - 'あ' as u32) }
        } else {
            c
        }
    }

    pub fn eq_ignore_kana_case(a: char, b: char) -> bool {
        StringUtil::to_hiragana(a) == StringUtil::to_hiragana(b)
    }

    pub fn hiragana_to_no_sign(c: char) -> char {
        //todo:改写成match
        // 'ぱ' => { 'は' }
        // 'ば' => { 'は' }
        // 'ぴ' => { 'ひ' }
        // 'び' => { 'ひ' }
        // 'ぷ' => { 'ふ' }
        // 'ぶ' => { 'ふ' }
        // 'ぺ' => { 'へ' }
        // 'べ' => { 'へ' }
        // 'ぽ' => { 'ほ' }
        // 'ぼ' => { 'ほ' }

        if HIRAGANA_G
            .chars()
            .chain(HIRAGANA_Z.chars())
            .chain(HIRAGANA_D.chars())
            .chain(HIRAGANA_B.chars())
            .contains(&c)
        {
            unsafe { char::from_u32_unchecked(c as u32 - 1) }
        } else if HIRAGANA_P.chars().contains(&c) {
            unsafe { char::from_u32_unchecked(c as u32 - 2) }
        } else {
            c
        }
    }
}

pub const KATAKANA_A: &str = "アカガサザタダナハバパマャヤラワ";
pub const KATAKANA_I: &str = "イキギシジチヂニヒビピミリ";
pub const KATAKANA_U: &str = "ウクグスズッツヅヌフブプムュユル";
pub const KATAKANA_E: &str = "エケゲセゼテデネヘベペメレ";
pub const KATAKANA_O: &str = "オコゴソゾトドノホボポモョヨロヲ";

pub const KATAKANA_K: &str = "カキクケコ";
pub const KATAKANA_S: &str = "サシスセソ";
pub const KATAKANA_T: &str = "タチツテト";
pub const KATAKANA_H: &str = "ハヒフヘホ";

pub const HIRAGANA_K: &str = "かきくけこ";
pub const HIRAGANA_G: &str = "がぎぐげご";

pub const HIRAGANA_S: &str = "さしすせそ";
pub const HIRAGANA_Z: &str = "ざじずぜぞ";

pub const HIRAGANA_T: &str = "たちつてと";
pub const HIRAGANA_D: &str = "だぢづでど";

pub const HIRAGANA_H: &str = "はひふへほ";
pub const HIRAGANA_B: &str = "ばびぶべぼ";
pub const HIRAGANA_P: &str = "ぱぴぷぺぽ";

const HIRAGANA: &str = "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろわをん";
const KATAKANA: &str = "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロワヲン";
const KATAKANA_MACRON: char = 'ー';

pub const KANJI_REPEAT: char = '々';


pub fn is_start_with<T: Eq>(vec: &Vec<T>, p: &Vec<T>) -> bool {
    for i in 0..p.len() {
        if vec.get(i) != p.get(i) {
            return false;
        }
    }
    true
}