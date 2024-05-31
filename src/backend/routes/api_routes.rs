use actix_web::web;

use crate::backend::handlers::api_handlers::{
    sentences_handlers::{
        get_all_sentences_askama, get_all_sentences_handler, get_random_sentence,
        new_sentence_handler,
    },
    words_handlers::{get_all_words_askama, get_all_words_handler, new_word_handler},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/new-word").route(web::post().to(new_word_handler)))
        .service(web::resource("/get-all-words").route(web::get().to(get_all_words_handler)))
        .service(web::resource("/get-all-words-askama").route(web::get().to(get_all_words_askama)))
        .service(web::resource("/new-sentence").route(web::post().to(new_sentence_handler)))
        .service(
            web::resource("/get-all-sentences").route(web::get().to(get_all_sentences_handler)),
        )
        .service(
            web::resource("/get-all-sentences-askama")
                .route(web::get().to(get_all_sentences_askama)),
        )
        .service(web::resource("/get-random-sentence").route(web::get().to(get_random_sentence)));
}
