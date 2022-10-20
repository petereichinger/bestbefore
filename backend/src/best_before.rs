use std::sync::Mutex;

use crate::models::FoodItem;
use actix_web::{
    get,
    web::{self, Data},
    Responder,
};

pub struct BestBeforeData {
    pub counter: u64,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    // let data = ;
}

pub async fn get_all_food_items(data: Data<Mutex<BestBeforeData>>) -> impl Responder {
    let mut data = data.lock().unwrap();

    data.counter += 3;
    println!("{}", data.counter);
    let item = vec![FoodItem::new(data.counter), FoodItem::new(data.counter)];
    web::Json(item)
}
