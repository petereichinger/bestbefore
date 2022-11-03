use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::macros::date;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, FromRow, Deserialize)]
pub struct FoodItem {
    id: i64,
    expiry: sqlx::types::time::Date,
    text: String,
}

impl FoodItem {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            expiry: date!(2022 - 10 - 18),
            text: "Lecker Food".into(),
        }
    }
}
