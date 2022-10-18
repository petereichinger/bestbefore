mod best_before;
mod config;
mod models;
mod server_error;

use actix_web::{App, HttpServer};
use server_error::ServerError;

#[actix_web::main]
async fn main() -> Result<(), ServerError> {
    let config = config::config()?;
    let port = config.get(config::HOST_KEY)?;
    HttpServer::new(|| App::new().configure(best_before::config))
        .bind(("localhost", port))?
        .run()
        .await?;

    Ok(())
}
