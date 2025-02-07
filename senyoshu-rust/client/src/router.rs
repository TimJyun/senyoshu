use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;

use senyoshu_common::types::integer::Integer;
use senyoshu_common::types::kanji_alias::Kanji;
use senyoshu_common::types::word::wid::WordIdentity;

use crate::page::about_page::AboutPage;
use crate::page::check_word_page::CheckWordPage;
use crate::page::collection_page::CollectionPage;
use crate::page::create_word_page::CreateWordPage;
use crate::page::diff_page::DiffPage;
use crate::page::editor_page::EditorPage;
use crate::page::glossary_page::{GlossaryFilter, GlossaryPage, Order};
use crate::page::home_page::HomePage;
use crate::page::inspection_page::InspectionPage;
use crate::page::kanji_list_page::KanjiListPage;
use crate::page::kanji_page::KanjiPage;
use crate::page::knowledge_page::KnowledgePage;
use crate::page::learn::learn_page::LearnPage;
use crate::page::login_page::LoginPage;
use crate::page::maintain::deduplicate_page::DeduplicatePage;
use crate::page::maintain::segment_page::SegmentPage;
use crate::page::management_page::ManagementPage;
use crate::page::setting_page::SettingPage;
use crate::page::voices_page::VoicesPage;
use crate::page::word_page::WordPage;
use crate::singleton::bottom_navigation::BottomNavigation;
use crate::singleton::top_navigation::{TopNavigation, TOP_NAVIGATION};
use crate::AppRoute::SegmentPage;

#[derive(Routable, PartialEq, Clone, Debug)]
#[rustfmt::skip]
pub enum AppRoute {
    #[layout(TopNavigation)]
    #[layout(BottomNavigation)]
    #[route("/home")]
    HomePage {},
    #[route("/learn")]
    LearnPage {},
    #[route("/collection")]
    CollectionPage {},
    #[end_layout]
    //
    //
    //
    #[route("/glossary?:filter&:order")]
    GlossaryPage {
        filter: GlossaryFilter,
        order: Order,
    },
    #[route("/login")]
    LoginPage {},
    #[route("/kanji_list/:name")]
    KanjiListPage { name: String },
    #[route("/kanji?:kanji")]
    KanjiPage { kanji: Kanji },
    #[route("/word?:wid")]
    WordPage { wid: WordIdentity },
    #[route("/check_word")]
    CheckWordPage {},
    #[route("/inspection?:pid")]
    InspectionPage { pid: Integer },
    #[route("/create_word")]
    CreateWordPage {},
    // #[route("/word_history?:pid")]
    // WordHistoryPage { pid: String },
    #[route("/voices")]
    VoicesPage {},
    #[route("/knowledge")]
    KnowledgePage {},
    #[route("/editor?:wid")]
    EditorPage { wid: WordIdentity },
    #[route("/about")]
    AboutPage {},
    #[route("/setting")]
    SettingPage {},
    #[route("/management")]
    ManagementPage {},
    #[route("/deduplicate")]
    DeduplicatePage {},
    #[route("/segment")]
    SegmentPage {},
    #[route("/diff?:wid&:wid2")]
    DiffPage { wid: WordIdentity, wid2: WordIdentity },
    //
    //
    //
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_no_back_button(true);

    let nav = use_navigator();
    let _ = use_coroutine(|_: UnboundedReceiver<()>| async move {
        sleep(Duration::from_secs(3)).await;
        nav.replace(AppRoute::LearnPage {});
    });

    rsx! {
        h1 { "Page Not Found" }
        div { "forward to learn page in 3 second" }
        div {
            Link { to: AppRoute::LearnPage {}, "click to learn" }
        }
        div {
            Link { to: AppRoute::HomePage {}, "click to home" }
        }
    }
}
