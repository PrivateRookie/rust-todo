use chrono::{NaiveDateTime, Utc};

pub mod api;

mod modles;
mod schema;

pub use modles::PathExtractor;

fn naivedate_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}