use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Identifiable};
use serde_derive::{Deserialize, Serialize};
use gotham_derive::{StateData, StaticResponseExtender};

use crate::db::schema::events;

#[derive(Queryable, Serialize, Debug)]
pub struct Event {
    pub id: i32,
    pub content: String,
    pub finished: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable, Deserialize)]
#[table_name = "events"]
pub struct NewEvent {
    pub content: String,
    pub finished: bool,
    #[serde(default = "super::naivedate_now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "super::naivedate_now")]
    pub updated_at: NaiveDateTime,
}


#[derive(Identifiable, Deserialize)]
#[table_name = "events"]
pub struct UpdateEventStatus {
    pub id : i32,
    pub finished: bool,
    #[serde(default = "super::naivedate_now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct PathExtractor {
    pub id: i32,
}