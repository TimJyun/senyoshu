use std::ops::Deref;

use serde::{Deserialize, Serialize};

use senyoshu_common::types::api::account::{
    Token, UserInfo, UserState, LOGIN_API, UPDATE_USER_STATE_API,
};
use senyoshu_common::util::passwd_hasher::get_passwd_hash;

use crate::storage::use_storage::GlobalSignalStorage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountInfo {
    pub user_info: UserInfo,
    pub token: Token,
}

pub struct Account(GlobalSignalStorage<Option<AccountInfo>>);

pub static ACCOUNT: Account = Account(GlobalSignalStorage::local(ACCOUNT_LOCAL_STORAGE, || None));

const ACCOUNT_LOCAL_STORAGE: &str = "account";

impl Account {
    pub fn peek(&self) -> Option<AccountInfo> {
        (*self.0.peek().deref()).to_owned()
    }

    pub fn snap(&self) -> Option<AccountInfo> {
        self.0.read().to_owned()
    }

    pub async fn login(&'static self, username: String, passwd: String) -> bool {
        let rv = async {
            let token = LOGIN_API
                .call(&(username, get_passwd_hash(&passwd)))
                .await
                .ok()??;
            let user_state = UPDATE_USER_STATE_API.call(&token).await.ok()?;
            match user_state {
                UserState::TokenRevoked => {
                    self.0.reset();
                    None
                }
                UserState::Valid(user_info) => {
                    let account_newest = AccountInfo { user_info, token };
                    *self.0.write() = Some(account_newest);
                    Some(())
                }
                UserState::Err => None,
            }
        }
        .await
        .is_some();
        rv
    }

    pub fn login_out(&self) {
        self.0.reset();
    }

    pub async fn refresh(&'static self) -> bool {
        async {
            let account_info_old = { self.0.peek().as_ref()?.to_owned() };
            let user_state = UPDATE_USER_STATE_API
                .call(&account_info_old.token)
                .await
                .ok()?;

            match user_state {
                UserState::TokenRevoked => {
                    self.0.reset();
                    None
                }
                UserState::Valid(user_info_new) => {
                    if user_info_new != account_info_old.user_info {
                        let account_info_new = AccountInfo {
                            user_info: user_info_new,
                            token: account_info_old.token,
                        };
                        *self.0.write() = Some(account_info_new);
                    }

                    Some(())
                }
                UserState::Err => None,
            }
        }
        .await
        .is_some()
    }
}
