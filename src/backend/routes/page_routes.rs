use actix_web::web;

use crate::backend::handlers::page_handlers::{
    index, manage_sentence_game, manage_word_game, sentence_game, sentences, word_game,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(index))
        .service(web::resource("/word-game").to(word_game))
        .service(web::resource("/manage-word-game").to(manage_word_game))
        .service(web::resource("/sentences").to(sentences))
        .service(web::resource("/sentence-game").to(sentence_game))
        .service(web::resource("/manage-sentence-game").to(manage_sentence_game));
}
