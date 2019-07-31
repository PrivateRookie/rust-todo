pub mod api;
pub mod modles;
pub mod schema;

use chrono::{NaiveDateTime, Utc};
use diesel::pg::PgConnection;

pub use modles::PathExtractor;

pub type Repo = gotham_middleware_diesel::Repo<PgConnection>;

fn naivedate_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

