use serde::Serialize;
use time::macros::date;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct FoodItem {
    id: u64,
    expiry: time::Date,
    text: String,
}

impl FoodItem {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            expiry: date!(2022 - 10 - 18),
            text: "Lecker Food".into(),
        }
    }
}
