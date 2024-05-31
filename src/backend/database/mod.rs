pub mod sentences;
pub mod words;

use log::{error, info};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
    Pool, Sqlite, SqlitePool,
};
use std::env;

use self::{sentences::create_sentences_tables, words::create_words_tables};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn create_db_connection() -> Result<sqlx::SqlitePool> {
    let db_path = env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");
    let options: SqliteConnectOptions = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool: Pool<Sqlite> = match SqlitePool::connect_with(options).await {
        Ok(pool) => {
            info!("Connected to the database");
            pool
        }
        Err(err) => {
            error!(
                "Error while creating or connecting to the database: {}",
                err
            );
            return Err(err.into());
        }
    };

    Ok(pool)
}

pub async fn create_all_tables(pool: &sqlx::SqlitePool) -> Result<()> {
    create_sentences_tables(pool).await?;
    create_words_tables(pool).await?;
    Ok(())
}
