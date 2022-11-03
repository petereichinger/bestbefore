use std::sync::{Arc, Mutex};

use actix_web::{get, post, web::Data, App, HttpResponse, Responder};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
struct BestBeforeState {
    pool: Pool<Sqlite>,
    counter: u8,
}

// pub async fn get_router() -> App<> {
//     info!("{}", std::env::current_dir().unwrap().display());

//     let pool = SqlitePoolOptions::new()
//         .max_connections(5)
//         .connect("sqlite://BestBefore.db")
//         .await
//         .unwrap();

//     // create_query

//     // sqlx::query!("select \"hello\"")
//     let shared_state = Arc::new(Mutex::new(BestBeforeState { pool }));

// }

#[get("/")]
async fn hello(state: Data<Mutex<BestBeforeState>>) -> impl Responder {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    HttpResponse::Ok().body(format!("{}", state.counter))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub fn config_app(cfg: &mut actix_web::web::ServiceConfig) {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_lazy("sqlite://BestBefore.db")
        .unwrap();

    let state = Data::new(Mutex::new(BestBeforeState { pool, counter: 0 }));
    cfg.app_data(state.clone()).service(hello).service(echo);
}
