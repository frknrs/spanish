use log::{error, info};
use serde::Serialize;

// For easier return
#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
#[derive(Serialize, Clone)]
pub struct Word {
    pub spanish: String,
    pub tipo: String,
    pub english: String,
}

pub async fn create_words_tables(pool: &sqlx::SqlitePool) -> Result<()> {
    let mut conn = pool.acquire().await.map_err(|err| {
        error!(
            "Failed to acquire connection while creating the tables: {}",
            err
        );
        Box::new(err) as Box<dyn std::error::Error>
    })?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS words (
            spanish TEXT NOT NULL,
            tipo TEXT NOT NULL,
            english TEXT NOT NULL
        )"
    )
    .execute(&mut *conn)
    .await
    .map_err(|err| {
        error!("Error while creating the words table: {}", err);
        Box::new(err) as Box<dyn std::error::Error>
    })?;

    info!("Words table created or already exists");

    Ok(())
}

impl Word {
    pub async fn insert_word(self, pool: &sqlx::SqlitePool) -> Result<()> {
        let mut conn = pool.acquire().await.map_err(|err| {
            error!(
                "Failed to acquire connection while creating the tables: {}",
                err
            );
            Box::new(err) as Box<dyn std::error::Error>
        })?;

        info!("Connected to the database for inserting the ad");

        sqlx::query!(
            "INSERT INTO words (spanish, tipo, english) VALUES (?, ?, ?)",
            self.spanish,
            self.tipo,
            self.english
        )
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            error!("Failed to insert ad to the database: {}", err);
            Box::new(err) as Box<dyn std::error::Error>
        })?;

        info!("Succesfully inserted the ad to the database");

        Ok(())
    }
    pub async fn get_all_words(pool: &sqlx::SqlitePool) -> Result<Vec<Word>> {
        let words = sqlx::query_as!(Word, "SELECT spanish, tipo, english FROM words")
            .fetch_all(pool)
            .await?;

        Ok(words)
    }
}
