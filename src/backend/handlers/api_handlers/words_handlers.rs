use actix_web::{web, HttpResponse};
use log::info;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::backend::database::words::Word;
use crate::backend::game::word_game::get_random_word;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct NewWord {
    spanish: String,
    tipo: String,
    english: String,
}

pub async fn new_word_handler(
    pool: web::Data<sqlx::SqlitePool>,
    form: web::Form<NewWord>,
) -> Result<HttpResponse> {
    info!("New word handler called with form data: {:?}", form);
    info!("new_word_handler called");

    let word = Word {
        spanish: form.spanish.clone(),
        tipo: form.tipo.clone(),
        english: form.english.clone(),
    };

    word.insert_word(&pool).await?;

    Ok(HttpResponse::Ok().content_type("text/html").body(""))
}

pub async fn get_all_words_handler(pool: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse> {
    let words = Word::get_all_words(&pool).await?;
    Ok(HttpResponse::Ok().json(words))
}

pub async fn get_random_word_handler(pool: web::Data<SqlitePool>) -> Result<HttpResponse> {
    get_random_word(pool).await
}
