#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate log;

mod api;
mod db;

use dotenv::dotenv;
use std::env;

use crate::api::router;

fn main() {
    let addr = "0.0.0.0:8000";
    dotenv().ok();
    env_logger::init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be specify");
    gotham::start(addr, router(db::Repo::new(&db_url)));
}
