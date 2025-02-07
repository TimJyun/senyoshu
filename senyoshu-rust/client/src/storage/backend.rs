use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

pub enum StorageError {}

pub trait IStorage {
    fn get<D: for<'de> Deserialize<'de>>(key: impl AsRef<str>) -> Option<D>;

    fn set<S: Serialize>(key: impl AsRef<str>, value: S) -> bool;
}

pub struct WebLocalStorage;

impl IStorage for WebLocalStorage {
    fn get<D: for<'de> Deserialize<'de>>(key: impl AsRef<str>) -> Option<D> {
        LocalStorage::get(key).ok()
    }

    fn set<S: Serialize>(key: impl AsRef<str>, value: S) -> bool {
        LocalStorage::set(key, value).is_ok()
    }
}
