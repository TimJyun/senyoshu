use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::storage::use_storage::GlobalSignalStorage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    pub silent_mode: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    pub show_refresh_app_button: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Zh,
    En,
    Ja,
}

impl Default for Setting {
    fn default() -> Setting {
        Setting {
            silent_mode: false,
            language: None,
            show_refresh_app_button: false,
        }
    }
}

const SETTING_KEY: &str = "setting";

impl Setting {
    pub fn get() -> Self {
        LocalStorage::get(SETTING_KEY).unwrap_or(Setting::default())
    }
}

pub static SETTING: GlobalSignalStorage<Setting> =
    GlobalSignalStorage::local(SETTING_KEY, || Setting::default());
