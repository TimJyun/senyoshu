use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::storage::account::ACCOUNT;
use crate::storage::dictionary::Dic;
use crate::storage::use_storage::GlobalSignalStorage;
use crate::storage::workbook::WorkBook;

pub mod account;
pub mod backend;
pub mod dictionary;
pub mod permanent_storage;
pub mod setting;
pub mod use_storage;
pub mod voice_setting;
pub mod workbook;

pub static LAST_UPDATED: GlobalSignalStorage<LastUpdated> =
    GlobalSignalStorage::local("last_updated", || LastUpdated {
        dic: None,
        workbook: None,
    });

#[derive(Default, Serialize, Deserialize)]
pub struct LastUpdated {
    pub dic: Option<DateTime<FixedOffset>>,
    pub workbook: Option<DateTime<FixedOffset>>,
}

pub async fn update() {
    let account = ACCOUNT.peek();
    if let Some(account) = account {
        if WorkBook::sync(account.token).await {
            debug!("WorkBook sync finish");
        } else {
            error!("WorkBook sync fail");
        };
    } else {
        debug!("no login");
    }

    Dic::update().await;
}
