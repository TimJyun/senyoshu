use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use dioxus::prelude::{GlobalSignal, Readable, ReadableRef, Write};
use gloo::storage::{LocalStorage, SessionStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};
use tracing::error;

pub struct GlobalSignalStorage<T: Serialize + DeserializeOwned + 'static> {
    key: &'static str,
    temporary: bool,
    init: fn() -> T,
    signal: GlobalSignal<Option<T>>,
}

pub struct GlobalSignalStorageReadBox<'a, T: Serialize + DeserializeOwned + 'static> {
    inner: ReadableRef<'a, GlobalSignal<Option<T>>>,
    unsync: PhantomUnsync,
    unsend: PhantomUnsend,
}

impl<T: Serialize + DeserializeOwned + 'static> Deref for GlobalSignalStorageReadBox<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T: Serialize + DeserializeOwned + 'static> GlobalSignalStorage<T> {
    pub const fn session(key: &'static str, init: fn() -> T) -> Self {
        Self {
            key,
            temporary: true,
            init,
            signal: GlobalSignal::new(|| None),
        }
    }
    pub const fn local(key: &'static str, init: fn() -> T) -> Self {
        Self {
            key,
            temporary: false,
            init,
            signal: GlobalSignal::new(|| None),
        }
    }

    pub fn reset(&self) {
        if self.temporary {
            SessionStorage::delete(self.key);
        } else {
            LocalStorage::delete(self.key);
        }
        let mut storage = self.signal.write();
        *storage = Some((self.init)());
    }

    fn load_from_storage(&self) -> T {
        if self.temporary {
            SessionStorage::get(&self.key)
        } else {
            LocalStorage::get(&self.key)
        }
        .unwrap_or_else(|_err| (self.init)())
    }

    pub fn read(&self) -> GlobalSignalStorageReadBox<T> {
        let is_none = self.signal.peek().is_none();
        if is_none {
            *self.signal.write() = Some(self.load_from_storage());
        }
        GlobalSignalStorageReadBox {
            inner: self.signal.read(),
            unsync: PhantomData,
            unsend: PhantomData,
        }
    }
    pub fn peek(&self) -> GlobalSignalStorageReadBox<T> {
        let is_none = self.signal.peek().is_none();
        if is_none {
            *self.signal.write() = Some(self.load_from_storage());
        }
        GlobalSignalStorageReadBox {
            inner: self.signal.peek(),
            unsync: PhantomData,
            unsend: PhantomData,
        }
    }
    pub fn write(&self) -> GlobalSignalStorageWriteBox<T> {
        let is_none = self.signal.peek().is_none();
        if is_none {
            *self.signal.write() = Some(self.load_from_storage());
        }

        GlobalSignalStorageWriteBox {
            key: self.key,
            temporary: self.temporary,
            inner: self.signal.write(),
            change: false,
            unsync: PhantomData,
            unsend: PhantomData,
        }
    }
}

pub struct GlobalSignalStorageWriteBox<T: Serialize + DeserializeOwned + 'static> {
    key: &'static str,
    temporary: bool,
    inner: Write<'static, Option<T>>,
    change: bool,
    unsync: PhantomUnsync,
    unsend: PhantomUnsend,
}

type PhantomUnsync = PhantomData<Cell<()>>;
type PhantomUnsend = PhantomData<MutexGuard<'static, ()>>;

impl<T: Serialize + DeserializeOwned + 'static> Deref for GlobalSignalStorageWriteBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T: Serialize + DeserializeOwned + 'static> DerefMut for GlobalSignalStorageWriteBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.change = true;
        self.inner.as_mut().unwrap()
    }
}

impl<T: Serialize + DeserializeOwned + 'static> Drop for GlobalSignalStorageWriteBox<T> {
    fn drop(&mut self) {
        if self.change {
            let result = if self.temporary {
                SessionStorage::set(&self.key, self.inner.as_ref().unwrap())
            } else {
                LocalStorage::set(&self.key, self.inner.as_ref().unwrap())
            };
            if let Err(error) = result {
                error!("StorageError:{error}")
            };
        }
    }
}
