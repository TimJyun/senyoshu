use std::io::Read;

use axum::Router;
use tokio::io::AsyncReadExt;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::util::SubscriberInitExt;

use senyoshu_common::types::api::account::{
    GET_OTHER_USER_INFO_API, LOGIN_API, REGISTER_API, UPDATE_PASSWD_API, UPDATE_USER_STATE_API,
};
use senyoshu_common::types::api::api::GET_SURF_SERVERS_API;
use senyoshu_common::types::api::dic::{CREATE_WORD_API, DELETE_WORD_API, GET_CHANGE_REQUEST_API, GET_WORD_BY_PID_API, GET_WORD_HISTORY_API, POST_WORD_API, SET_ADOPTED_API, SYNC_DIC_API, UPDATE_MANY_API};
use senyoshu_common::types::api::learn::{GET_RECORD_API, POST_LEARN_RECORD_API};

use crate::api::account::get_other_user_info::get_other_user_info_api;
use crate::api::account::login::login_api;
use crate::api::account::register::register_api;
use crate::api::account::update_passwd::update_passwd_api;
use crate::api::account::update_user_state::update_user_state_api;
use crate::api::AxumAPi;
use crate::api::dic::create_word::create_word_api;
use crate::api::dic::delete_word::delete_word_api;
use crate::api::dic::get_change_request::get_change_request_api;
use crate::api::dic::get_word_by_pid::get_word_by_pid_api;
use crate::api::dic::get_word_history::get_word_history_api;
use crate::api::dic::post_word::post_word_api;
use crate::api::dic::set_adopted::set_adopted_api;
use crate::api::dic::sync_dic::sync_dic_api;
use crate::api::dic::update_many::update_many_api;
use crate::api::get_surf_servers::get_surf_servers_api;
use crate::api::learn::get_record::get_record_api;
use crate::api::learn::post_record::post_learn_record_api;
use crate::database::database::{GlobalDatabase, TEST_DATABASE};

mod api;
mod database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    GlobalDatabase::init_database(TEST_DATABASE, false).await;

    let cors = CorsLayer::permissive();
    let app = Router::new();
    let app = app
        .nest_service("", ServeDir::new("../target/dist").fallback(ServeFile::new("../target/dist/index.html")))
        //account
        .set_api_handle(UPDATE_USER_STATE_API, update_user_state_api)
        .set_api_handle(GET_OTHER_USER_INFO_API, get_other_user_info_api)
        .set_api_handle(LOGIN_API, login_api)
        .set_api_handle(REGISTER_API, register_api)
        .set_api_handle(UPDATE_PASSWD_API, update_passwd_api)
        //dic
        .set_api_handle(CREATE_WORD_API, create_word_api)
        .set_api_handle(DELETE_WORD_API, delete_word_api)
        .set_api_handle(SYNC_DIC_API, sync_dic_api)
        .set_api_handle(GET_CHANGE_REQUEST_API, get_change_request_api)
        .set_api_handle(GET_WORD_BY_PID_API, get_word_by_pid_api)
        .set_api_handle(GET_WORD_HISTORY_API, get_word_history_api)
        .set_api_handle(POST_WORD_API, post_word_api)
        .set_api_handle(SET_ADOPTED_API, set_adopted_api)
        .set_api_handle(UPDATE_MANY_API, update_many_api)
        //learn
        .set_api_handle(POST_LEARN_RECORD_API, post_learn_record_api)
        .set_api_handle(GET_RECORD_API, get_record_api)
        //surf
        .set_api_handle(GET_SURF_SERVERS_API, get_surf_servers_api)
        //other settings
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .into_make_service();


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
