use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::embed::{Asset, YO_MI_FILE};

#[derive(Serialize, Deserialize)]
pub struct Yomi {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kun: Vec<String>,
}

pub static YO_MI_MAP: Lazy<HashMap<char, Yomi>> = Lazy::new(|| {
    let yo_mi_file = Asset::get(YO_MI_FILE).unwrap();
    serde_json::from_slice(yo_mi_file.data.as_ref()).unwrap()
});
