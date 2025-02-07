use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod account;
pub mod api;
pub mod dic;
pub mod learn;
pub mod session;

pub struct API<REQ: Serialize + DeserializeOwned, RES: Serialize + DeserializeOwned> {
    name: &'static str,
    request: PhantomData<REQ>,
    response: PhantomData<RES>,
}

impl<REQ: Serialize + DeserializeOwned, RES: Serialize + DeserializeOwned> API<REQ, RES> {
    pub fn path(&self) -> String {
        format!("/api/{}", self.name)
    }

    const fn new(path: &'static str) -> API<REQ, RES> {
        API {
            name: path,
            request: PhantomData,
            response: PhantomData,
        }
    }

    pub async fn call(&self, body: &REQ) -> Result<RES, reqwest::Error> {
        let client = reqwest::Client::new();
        let host = get_host();
        let host = host.trim_end_matches("/");
        let url = format!("{host}/api/{}", self.name);

        client
            .post(url)
            .json(body)
            .send()
            .await?
            .json::<RES>()
            .await
    }
}


pub fn get_host() -> String {
    if cfg!(target_family = "wasm") && cfg!(feature = "android") == false {
        if let Some(window) = web_sys::window() {
            let location = window.location();
            if let (Ok(protocol), Ok(host)) = (location.protocol(), location.host()) {
                format!("{protocol}//{host}/")
            } else {
                String::from("https://senyoshu.com/")
            }
        } else {
            String::from("https://senyoshu.com/")
        }
    } else {
        String::from("https://senyoshu.com/")
    }
}