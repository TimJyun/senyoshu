use std::sync::OnceLock;
use std::time::Duration;

use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Schema};
use tokio::time::sleep;

use crate::database::{account, learn};
use crate::database::dic::{sounds, word_history, words};

pub(crate) static GLOBAL_DATABASE: OnceLock<DatabaseConnection> = OnceLock::new();

pub struct GlobalDatabase;

pub const TEST_DATABASE: &str =
    "postgresql://postgres:xxx@yyy:zzz/postgres";

impl GlobalDatabase {
    pub async fn init_database(url: &str, create_table: bool) {
        let mut db = None;
        for i in 0..10u64 {
            sleep(Duration::from_secs(5 * i)).await;
            db = Database::connect(url).await.ok();
            if db.is_some() {
                break;
            }
        }
        let db = db.unwrap();
        if create_table {
            let db_postgres = DbBackend::Postgres;
            let schema = Schema::new(db_postgres);
            let _ = db
                .execute(db_postgres.build(&schema.create_table_from_entity(account::Entity)))
                .await;
            let _ = db
                .execute(db_postgres.build(&schema.create_table_from_entity(word_history::Entity)))
                .await;
            let _ = db
                .execute(db_postgres.build(&schema.create_table_from_entity(words::Entity)))
                .await;
            let _ = db
                .execute(db_postgres.build(&schema.create_table_from_entity(learn::Entity)))
                .await;
            let _ = db
                .execute(db_postgres.build(&schema.create_table_from_entity(sounds::Entity)))
                .await;
        }

        GLOBAL_DATABASE.get_or_init(move || db);
    }
}
