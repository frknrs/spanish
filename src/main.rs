pub mod backend {
    pub mod database;
    pub mod handlers;
    pub mod routes;
}

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use crate::backend::database::{create_all_tables, create_db_connection};
use crate::backend::routes::{api_routes, page_routes};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info) // Ensure info level messages are logged
        .init();

    dotenv().ok();
    println!("Hello, world!");
    let pool = create_db_connection().await?;
    create_all_tables(&pool).await?;

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
            .configure(api_routes::configure)
            .configure(page_routes::configure)
            .service(actix_files::Files::new("/", "src/frontend/static").show_files_listing())
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await?;

    Ok(())
}
