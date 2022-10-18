use serde::Serialize;
use time::macros::date;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct FoodItem {
    expiry: time::Date,
    text: String,
}

impl FoodItem {
    pub fn new() -> Self {
        Self {
            expiry: date!(2022 - 10 - 18),
            text: "Lecker Food".into(),
        }
    }
}
