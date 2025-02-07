use std::future::Future;

use axum::{Json, Router};
use axum::handler::Handler;
use axum::routing::post;
use serde::de::DeserializeOwned;
use serde::Serialize;

use senyoshu_common::types::api::API;

pub(crate) mod account;
pub(crate) mod dic;
pub(crate) mod learn;
pub(crate) mod get_surf_servers;

pub trait AxumAPi<S> {
    fn set_api_handle<REQ, RES, H, Fut, T>(self, api: API<REQ, RES>, handle: H) -> Self
        where
            REQ: Serialize + DeserializeOwned,
            RES: Serialize + DeserializeOwned,
            H: Handler<T, S> + Fn(Json<REQ>) -> Fut,
            T: 'static,
            Fut: Future<Output=Json<RES>>;
}

impl<S> AxumAPi<S> for Router<S>
    where
        S: Clone + Send + Sync + 'static,
{
    fn set_api_handle<REQ, RES, H, Fut, T>(self, api: API<REQ, RES>, handle: H) -> Self
        where
            REQ: Serialize + DeserializeOwned,
            RES: Serialize + DeserializeOwned,
            H: Handler<T, S> + Fn(Json<REQ>) -> Fut,
            T: 'static,
            Fut: Future<Output=Json<RES>>,
    {
        self.route(api.path().as_str(), post(handle))
    }
}
