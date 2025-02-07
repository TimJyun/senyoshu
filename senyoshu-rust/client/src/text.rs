use dioxus::prelude::{GlobalMemo, Signal};

use crate::i18n::cn::CN_TEXT;
use crate::i18n::en::EN_TEXT;
use crate::storage::setting::{Language, SETTING};
use crate::window::WINDOW_LANGUAGE;

pub static TEXT: GlobalMemo<Text> = Signal::global_memo(|| {
    let language = SETTING
        .read()
        .language
        .unwrap_or_else(|| WINDOW_LANGUAGE.unwrap_or(Language::Zh));

    match language {
        Language::Zh => CN_TEXT,
        Language::En => EN_TEXT,
        // todo: 日语界面文本
        // Language::Ja => JA_TEXT,
        Language::Ja => CN_TEXT,
    }
});

#[derive(PartialEq)]
pub struct Text {
    pub editor_page_action_submit: &'static str,
    pub editor_page_action_submit_then_pass: &'static str,

    pub glossary_page_selector_select_all: &'static str,
    pub glossary_page_selector_clear: &'static str,
    pub glossary_page_action_add_to_plan: &'static str,

    pub login_page_username: &'static str,
    pub login_page_password: &'static str,
    pub login_page_register: &'static str,
    pub login_page_sign_in: &'static str,

    pub voice_page_action_try_listen: &'static str,
    pub voice_page_action_save_setting: &'static str,
    pub voice_page_action_reset_to_default: &'static str,
    pub voice_page_global_volume: &'static str,
    pub voice_page_example_word: &'static str,

    pub word_page_action_delete: &'static str,
    pub word_page_action_edite: &'static str,

    pub kanji_list_page_selector_select_all: &'static str,
    pub kanji_list_page_selector_clear: &'static str,
    pub kanji_list_page_action_reload: &'static str,
    pub kanji_list_page_action_save_plan: &'static str,

    pub knowledge_page_filter_hide_freezed: &'static str,
    pub knowledge_page_filter_show_freezed: &'static str,
    pub knowledge_page_filter_hide_unfreezed: &'static str,
    pub knowledge_page_filter_show_unfreezed: &'static str,
    pub knowledge_page_filter_hide_kanji: &'static str,
    pub knowledge_page_filter_show_kanji: &'static str,
    pub knowledge_page_filter_hide_txt: &'static str,
    pub knowledge_page_filter_show_txt: &'static str,
    pub knowledge_page_filter_hide_kana: &'static str,
    pub knowledge_page_filter_show_kana: &'static str,

    pub knowledge_page_selector_reload: &'static str,
    pub knowledge_page_selector_reset: &'static str,
    pub knowledge_page_selector_select_all: &'static str,

    pub knowledge_page_selector_clear: &'static str,
    pub knowledge_page_action_freeze: &'static str,
    pub knowledge_page_action_unfreeze: &'static str,

    pub home_page_to_create_word_page: &'static str,
    pub home_page_to_check_page: &'static str,

    pub home_page_to_login_in_now_button_hint: &'static str,
    pub home_page_to_login_in_now_button: &'static str,
    pub home_page_to_voices_page: &'static str,
    pub home_page_to_knowledge_page: &'static str,
    pub home_page_to_management_page: &'static str,
    pub home_page_to_setting_page: &'static str,
    pub home_page_to_about_page: &'static str,
    pub home_page_sign_out: &'static str,
    pub home_page_connect_to_japan_internet: &'static str,

    pub management_page_download_dic: &'static str,
    pub management_page_to_deduplicate_page: &'static str,

    pub setting_page_menu_show_refresh_app: &'static str,
}
