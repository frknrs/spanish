use actix_web::{web, HttpResponse};
use askama::Template;
use log::info;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

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

#[derive(Template)]
#[template(path = "sentence-game-logic.html")]
struct SentenceGameTemplate {
    shuffled_sentence: String,
    correct_sentence: String,
}

#[derive(Serialize)]
struct RandomSentence {
    shuffled_sentence: String,
    correct_sentence: String,
}

pub async fn get_random_sentence(pool: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse> {
    let sentence = Sentence::get_random_sentence(&pool).await?;
    let correct_sentence = sentence.sentence.clone();
    let mut words: Vec<String> = sentence
        .sentence
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    words.shuffle(&mut rand::thread_rng());
    let shuffled_sentence = words.join(" ");

    let response = RandomSentence {
        shuffled_sentence,
        correct_sentence,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn check_sentence(form: web::Form<CheckSentence>) -> Result<HttpResponse> {
    let user_sentence = form.user_sentence.trim();
    let correct_sentence = form.correct_sentence.trim();

    info!("Received form data: {:?}", form);

    if user_sentence == correct_sentence {
        Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("Correct!"))
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("Incorrect!"))
    }
}

#[derive(Deserialize, Debug)]
pub struct CheckSentence {
    pub user_sentence: String,
    pub correct_sentence: String,
}
