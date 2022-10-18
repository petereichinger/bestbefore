use crate::models::FoodItem;
use actix_web::{get, web, Responder};
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/bestBefore/v1").service(food));
}

#[get("/food")]
async fn food() -> impl Responder {
    let item = FoodItem::new();
    web::Json(item)
}
