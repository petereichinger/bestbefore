mod best_before;
mod config;
mod models;
mod server_error;

use std::sync::Mutex;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use best_before::{get_all_food_items, BestBeforeData};
use server_error::ServerError;

#[actix_web::main]
async fn main() -> Result<(), ServerError> {
    let config = config::config()?;
    let port = config.get(config::HOST_KEY)?;
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Mutex::new(BestBeforeData { counter: 0 })))
            .route("/bestBefore/v1/foodItem", web::get().to(get_all_food_items))
    })
    .bind(("localhost", port))?
    .run()
    .await?;

    Ok(())
}
