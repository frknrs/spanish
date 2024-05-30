pub mod backend {
    pub mod database;
    pub mod game;
    pub mod handlers;
}

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;

use crate::backend::handlers::api_handlers::words_handlers::{
    get_all_words_handler, get_random_word_handler,
};
use crate::backend::{
    database::{create_db_connection, words::create_words_tables},
    handlers::api_handlers::words_handlers::new_word_handler,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info) // Ensure info level messages are logged
        .init();

    dotenv().ok();
    println!("Hello, world!");
    let pool = create_db_connection().await?;
    create_words_tables(&pool).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:8000") // Update to match the frontend's origin
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"]) // Include OPTIONS
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(web::resource("/").to(index))
            .service(web::resource("/word-game").to(word_game))
            .service(web::resource("/manage-word-game").to(manage_word_game))
            .service(web::resource("/new-word").route(web::post().to(new_word_handler)))
            .service(web::resource("/get-all-words").route(web::get().to(get_all_words_handler)))
            .service(
                web::resource("/get-random-word").route(web::get().to(get_random_word_handler)),
            )
            .service(actix_files::Files::new("/", "src/frontend/static").show_files_listing())
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await?;

    Ok(())

    // HttpServer::new(|| {
    //     App::new()
    //         // .configure(handlers::api_handlers::config)
    //         // .configure(handlers::page_handlers::config)
    //         .service(web::resource("/").to(index))
    //         .service(web::resource("/word-game").to(word_game))
    //         .service(web::resource("/manage-word-game").to(manage_word_game))
    //         .service(actix_files::Files::new("/", "src/frontend/static").show_files_listing())
    //         .service(web::resource("/sign_up").route(web::post().to(new_word_handler)))
    // })
    // .bind("127.0.0.1:5000")?
    // .run()
    // .await?;

    // Ok(())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../src/frontend/static/pages/index.html"))
}

async fn word_game() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../src/frontend/static/pages/word-game.html"))
}

async fn manage_word_game() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!(
            "../src/frontend/static/pages/manage-word-game.html"
        ))
}
