mod routes;
mod models;
mod utils;

use actix_web::{middleware::Logger, App, HttpServer};
use crate::routes::register_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("Axonix AI backend running on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(register_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
