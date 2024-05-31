use log::{error, info};
use serde::Serialize;

#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Clone)]
pub struct Sentence {
    pub sentence: String,
}

pub async fn create_sentences_tables(pool: &sqlx::SqlitePool) -> Result<()> {
    let mut conn = pool.acquire().await.map_err(|err| {
        error!(
            "Failed to acquire connection while creating the tables: {}",
            err
        );
        Box::new(err) as Box<dyn std::error::Error>
    })?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS sentences (
            sentence TEXT NOT NULL
        )"
    )
    .execute(&mut *conn)
    .await
    .map_err(|err| {
        error!("Error while creating the words table: {}", err);
        Box::new(err) as Box<dyn std::error::Error>
    })?;

    info!("Sentences table created or already exists");

    Ok(())
}

impl Sentence {
    pub async fn insert_sentence(self, pool: &sqlx::SqlitePool) -> Result<()> {
        let mut conn = pool.acquire().await.map_err(|err| {
            error!(
                "Failed to acquire connection while inserting the sentence: {}",
                err
            );
            Box::new(err) as Box<dyn std::error::Error>
        })?;

        info!("Connected to the database for inserting the sentence");

        sqlx::query!("INSERT INTO sentences (sentence) VALUES (?)", self.sentence,)
            .execute(&mut *conn)
            .await
            .map_err(|err| {
                error!("Failed to insert sentence to the database: {}", err);
                Box::new(err) as Box<dyn std::error::Error>
            })?;

        info!("Succesfully inserted the sentence to the database");

        Ok(())
    }
    pub async fn get_all_sentences(pool: &sqlx::SqlitePool) -> Result<Vec<Sentence>> {
        let sentences = sqlx::query_as!(Sentence, "SELECT sentence FROM sentences")
            .fetch_all(pool)
            .await?;

        Ok(sentences)
    }

    pub async fn get_random_sentence(pool: &sqlx::SqlitePool) -> Result<Sentence> {
        let sentence = sqlx::query_as!(
            Sentence,
            "SELECT sentence FROM sentences ORDER BY RANDOM() LIMIT 1"
        )
        .fetch_one(pool)
        .await?;

        Ok(sentence)
    }
}
