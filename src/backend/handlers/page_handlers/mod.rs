use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../../frontend/static/index.html"))
}

pub async fn word_game() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../../frontend/static/word-game.html"))
}

pub async fn manage_word_game() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!(
            "../../../frontend/static/manage-word-game.html"
        ))
}

pub async fn sentence_game() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../../frontend/static/sentence-game.html"))
}

pub async fn manage_sentence_game() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!(
            "../../../frontend/static/manage-sentence-game.html"
        ))
}

pub async fn sentences() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../../../templates/sentences.html"))
}
