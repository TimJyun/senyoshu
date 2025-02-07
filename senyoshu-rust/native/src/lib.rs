use std::future::Future;
use std::sync::RwLock;

use derive_more::Deref;
use once_cell::sync::Lazy;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;
use tun2proxy::CancellationToken;

pub use init::init;
pub use surfing::is_surfing;
pub use surfing::stop_surfing;
pub use surfing::surfing;
pub use surfing::ConfigLite;

pub mod init;
pub mod surfing;
pub(crate) mod tun_device;

uniffi::setup_scaffolding!("native");

pub(crate) static HANDLES: HandlesBox = HandlesBox(RwLock::new(Handles {
    runtime: Lazy::new(|| {
        Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .thread_name("senyoshu-core")
            .thread_stack_size(2 * 1024 * 1024)
            .build()
            .unwrap()
    }),
    ss: None,
    tun2proxy: None,
    tun2proxy_token: None,
}));

#[derive(Deref)]
pub(crate) struct HandlesBox(RwLock<Handles>);

pub(crate) struct Handles {
    runtime: Lazy<Runtime>,
    ss: Option<JoinHandle<()>>,
    tun2proxy: Option<JoinHandle<()>>,
    tun2proxy_token: Option<CancellationToken>,
}

impl HandlesBox {
    pub(crate) fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.0.read().unwrap().runtime.spawn(future)
    }
}
