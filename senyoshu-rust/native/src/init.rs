use std::fmt::Debug;
use std::sync::OnceLock;

use once_cell::sync::Lazy;
use redb::{Database, TableDefinition};
use tracing::log::LevelFilter;

#[uniffi::export]
pub fn init(home_path: String) {
    use android_logger::Config;
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));
    PATH.get_or_init(|| home_path);
}

pub static PATH: OnceLock<String> = OnceLock::new();
pub static REDB: Lazy<Database> = Lazy::new(|| {
    let redb_path = PATH.get().unwrap().trim_end_matches("/").to_string() + "/data.db";
    Database::create(redb_path).expect("redb init fail")
});

pub const TABLE: TableDefinition<&str, String> = TableDefinition::new("global");
