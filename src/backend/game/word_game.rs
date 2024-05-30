use actix_web::{web, HttpResponse};
use rand::prelude::SliceRandom;
use serde::Serialize;
use sqlx::SqlitePool;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use crate::backend::database::words::Word;

#[derive(Serialize)]
struct RandomWord {
    pub spanish_word: String,
    pub word_type: String,
    pub english_options: Vec<String>,
}

pub async fn get_random_word(pool: web::Data<SqlitePool>) -> Result<HttpResponse> {
    let words = Word::get_all_words(&pool).await?;
    let random_word = words.choose(&mut rand::thread_rng()).unwrap();

    let mut rng = rand::thread_rng();

    // Select 4 random types and include the matched type
    let mut types: Vec<String> = words.iter().map(|w| w.tipo.clone()).collect();
    types.shuffle(&mut rng);
    types.truncate(4);
    types.push(random_word.tipo.clone());
    types.shuffle(&mut rng);

    // Select 4 random English words and include the matched English word
    let mut english_options: Vec<String> = words.iter().map(|w| w.english.clone()).collect();
    english_options.shuffle(&mut rng);
    english_options.truncate(4);
    english_options.push(random_word.english.clone());
    english_options.shuffle(&mut rng);

    // Format the response as HTML
    let types_html = types
        .iter()
        .map(|t| format!("<div class='selection'><p>{}</p></div>", t))
        .collect::<Vec<String>>()
        .join("");
    let options_html = english_options
        .iter()
        .map(|e| format!("<div class='selection'><p>{}</p></div>", e))
        .collect::<Vec<String>>()
        .join("");

    let full_html = format!(
        "<div class='current-spanish-word'><h2>{}</h2></div><div class='type-section'><div class='options'>{}</div></div><div class='translate-section'><div class='options'>{}</div></div>",
        random_word.spanish, types_html, options_html
    );

    Ok(HttpResponse::Ok().body(full_html))
}
