use actix_challenge::*;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = init_database(10);
    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route(
                "/update_username/<username>",
                web::post().to(update_username),
            )
            .route(
                "/update_password/<password>",
                web::post().to(update_password),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
