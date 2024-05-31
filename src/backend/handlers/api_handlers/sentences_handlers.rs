use actix_web::{web, HttpResponse};
use askama::Template;
use log::info;
use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::backend::database::sentences::Sentence;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct NewSentence {
    sentence: String,
}

pub async fn new_sentence_handler(
    pool: web::Data<sqlx::SqlitePool>,
    form: web::Form<NewSentence>,
) -> Result<HttpResponse> {
    info!("New sentence handler called with form data: {:?}", form);
    info!("new_sentence_handler called");

    let sentence = Sentence {
        sentence: form.sentence.clone(),
    };

    sentence.insert_sentence(&pool).await?;

    Ok(HttpResponse::Ok().content_type("text/html").body(""))
}

pub async fn get_all_sentences_handler(pool: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse> {
    let sentences = Sentence::get_all_sentences(&pool).await?;
    Ok(HttpResponse::Ok().json(sentences))
}

#[derive(Template)]
#[template(path = "sentences.html")]
#[allow(dead_code)]
struct SentencesTemplate {
    sentences: Vec<Sentence>,
}

pub async fn get_all_sentences_askama(pool: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse> {
    let sentences = Sentence::get_all_sentences(&pool).await?;
    let template = SentencesTemplate { sentences };
    let rendered = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "sentence-game-logic.html")]
struct SentenceGameTemplate {
    shuffled_sentence: String,
    words: Vec<String>,
}

pub async fn get_random_sentence(pool: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse> {
    let sentence = Sentence::get_random_sentence(&pool).await?;
    let mut words: Vec<String> = sentence
        .sentence
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    words.shuffle(&mut rand::thread_rng());
    let shuffled_sentence = words.join(" ");

    let template = SentenceGameTemplate {
        shuffled_sentence,
        words,
    };
    let rendered = template.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
