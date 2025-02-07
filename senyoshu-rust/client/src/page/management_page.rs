use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;
use std::ops::{Deref, DerefMut};

use dioxus::core_macro::rsx;
use dioxus::html::set;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use gloo::console::log;
use itertools::Itertools;
use smallvec::SmallVec;
use tracing::debug;

use senyoshu_common::glossary::jo_yo_kan_ji::YO_MI_MAP;
use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::api::dic::{CREATE_WORD_API, UPDATE_MANY_API};
use senyoshu_common::types::word::parts_of_speech::{
    Compound, PartsOfSpeech, VerbClass, VerbConjugation,
};
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word::WordElement;
use senyoshu_common::types::word::word_entry::WordDefine;
use senyoshu_common::util::string_util::{is_start_with, StringUtil, HIRAGANA_K};

use crate::components::button::Button;
use crate::file::download;
use crate::global::BUSYING;
use crate::imgs::FORWARD_12_12;
use crate::router::AppRoute;
use crate::storage::account::ACCOUNT;
use crate::storage::dictionary::{Dic, DicModel, DIC};
use crate::text::TEXT;

async fn update_many_api(dic: &mut DicModel, token: Token) -> bool {
    let mut maps_vec = Vec::new();
    for (idx, (wid, wd)) in dic.iter().enumerate() {
        if idx % 500 == 0 {
            maps_vec.push(HashMap::default());
        }
        let last = maps_vec.last_mut().unwrap();
        last.insert(wid.to_owned(), wd.to_owned());
    }
    let mut result = true;
    for maps in maps_vec.into_iter() {
        let this_result = UPDATE_MANY_API
            .call(&(token.to_owned(), maps.to_owned()))
            .await
            .unwrap_or(false);

        if this_result {
            for key in maps.keys() {
                dic.remove(key);
            }
        } else {
            result = false
        }
    }
    if result {
        debug!("update many success");
    } else {
        debug!("update many failed");
    };

    true
}

pub fn ManagementPage() -> Element {
    let mut update_many = use_signal(|| HashMap::<WordIdentity, WordDefine>::new());
    let update_many_disabled = update_many.read().is_empty() || *BUSYING.read();

    let mut post_many = use_signal(|| Vec::<WordDefine>::new());
    let post_many_disabled = post_many.read().is_empty() || *BUSYING.read();

    let account_info = ACCOUNT.snap()?;

    rsx! {
        div { style: "margin:16px",
            Link { to: AppRoute::CreateWordPage {},
                { TEXT.read().home_page_to_create_word_page},
                img { style: "margin-right: 4px", src: FORWARD_12_12 }
            }
        }
        div { style: "margin:16px",
            Link { to: AppRoute::CheckWordPage {},
                {TEXT.read().home_page_to_check_page},
                img { style: "margin-right: 4px", src: FORWARD_12_12 }
            }
        }
        div { style: "margin:16px",
            Link { to: AppRoute::DeduplicatePage {},
                {TEXT.read().management_page_to_deduplicate_page},
                img { style: "margin-right: 4px", src: FORWARD_12_12 }
            }
        }
        div { style: "margin:16px",
            Link { to: AppRoute::SegmentPage {},
                "单词分段"
                img { style: "margin-right: 4px", src: FORWARD_12_12 }
            }
        }
        div {
            style: "margin:16px",
            onclick: |_| {
                let dic_map = DIC.peek();
                let dic_map_ref = dic_map.deref().deref();
                let dic_json = serde_json::to_string(dic_map_ref).unwrap();
                download(dic_json, "dic.json".to_string());
            },
            {TEXT.read().management_page_download_dic}
        }

        div {
            Button {
                disabled: *BUSYING.read(),
                onclick: {
                    let token = account_info.token.to_owned();
                    move |_| {
                        *BUSYING.write() = true;
                        let token = token.to_owned();
                        spawn(async move {
                            if Dic::update().await {
                                let mut update_many_map = deal_words();
                                if update_many_map.len() == 0 {
                                    debug!("custom update nothing");
                                } else if update_many_api(&mut update_many_map, token).await {
                                    debug!("custom update success");
                                } else {
                                    debug!("custom update failed");
                                };
                            }
                            *BUSYING.write() = false;
                        });
                    }
                },
                "自定义更新"
            }
        }
        div {
            Button {
                disabled: update_many_disabled,
                onclick: {
                    let token = account_info.token.to_owned();
                    move |_| {
                        *BUSYING.write() = true;
                        let token = token.to_owned();
                        spawn(async move {
                            let mut update_many_map = { update_many.read().to_owned() };
                            if update_many_api(&mut update_many_map, token).await {
                                debug!("update all success");
                            } else {
                                debug!("update some failed");
                            };
                            update_many.set(update_many_map);
                            *BUSYING.write() = false;
                        });
                    }
                },
                "批量更新"
            }
            "count:  {update_many.read().len()}"
            input {
                r#type: "file",
                accept: ".json",
                disabled: *BUSYING.read(),
                onchange: move |evt| {
                    if let Some(file_engine) = &evt.files() {
                        let files = file_engine.files();
                        for file_name in files {
                            let file_engine = file_engine.to_owned();
                            spawn(async move {
                                if let Some(file_str) = file_engine
                                    .read_file_to_string(file_name.as_str())
                                    .await
                                {
                                    if let Ok(loaded) = serde_json::from_str::<
                                        HashMap<WordIdentity, WordDefine>,
                                    >(file_str.as_str()) {
                                        let mut update_many_write = update_many.write();
                                        for (k, v) in loaded.into_iter() {
                                            update_many_write.insert(k, v);
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
        }
        div {
            Button {
                disabled: post_many_disabled,
                onclick: {
                    let token = account_info.token.to_owned();
                    move |_| {
                        *BUSYING.write() = true;
                        let token = token.to_owned();
                        spawn(async move {
                            while let Some(word_define) = {
                                let mut post_many_write = post_many.write();
                                let wd = post_many_write.pop();
                                drop(post_many_write);
                                wd
                            } {
                                let param = (token.to_owned(), word_define.to_owned());
                                let result = CREATE_WORD_API.call(&param).await;
                                if let Ok(Some(wid)) = result {} else {
                                    let mut post_many_write = post_many.write();
                                    post_many_write.push(word_define);
                                    drop(post_many_write);
                                }
                            }
                            *BUSYING.write() = false;
                        });
                    }
                },
                "批量上传"
            }
            "count:  {post_many.read().len()}"
            input {
                r#type: "file",
                accept: ".json",
                disabled: *BUSYING.read(),
                onchange: move |evt| {
                    if let Some(file_engine) = &evt.files() {
                        let files = file_engine.files();
                        for file_name in files {
                            let file_engine = file_engine.to_owned();
                            spawn(async move {
                                if let Some(file_str) = file_engine
                                    .read_file_to_string(file_name.as_str())
                                    .await
                                {
                                    if let Ok(loaded) = serde_json::from_str::<
                                        Vec<WordDefine>,
                                    >(file_str.as_str()) {
                                        let mut post_many_write = post_many.write();
                                        for wd in loaded.into_iter() {
                                            post_many_write.push(wd);
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
        }
    }
}

fn deal_words() -> DicModel {
    let mut new_map = Dic::get();
    let old_dic = Dic::get();
    new_map.retain(|wid, wd| {
        if wd.word.get_txt() != old_dic.get(wid).unwrap().word.get_txt() {
            log!("txt:{}", wd.word.get_txt());
            panic!("txt-old:{}", old_dic.get(wid).unwrap().word.get_txt());
        };

        if wd.word.get_ruby() != old_dic.get(wid).unwrap().word.get_ruby() {
            log!("ruby:{}", wd.word.get_ruby());
            panic!("ruby-old:{}", old_dic.get(wid).unwrap().word.get_ruby());
        };

        old_dic.get(wid) != Some(wd)
    });
    debug!("len:{}", new_map.iter().count());

    new_map
    // ;Default::default()
}
