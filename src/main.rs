#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate env_logger;

use dotenv::dotenv;
use gotham::pipeline::{new_pipeline, single::single_pipeline};
use gotham::router::{builder::*, Router};
use gotham_middleware_diesel::DieselMiddleware;
use std::env;

mod db;

fn router(repo: db::api::Repo) -> Router {
    let (chain, pipeline) =
        single_pipeline(new_pipeline().add(DieselMiddleware::new(repo)).build());

    build_router(chain, pipeline, |route| {
        route.get("/").to(db::api::event_list);
        route.post("/").to(db::api::event_post);
        route.put("/").to(db::api::update_status);
        route.get("/:id").with_path_extractor::<db::PathExtractor>().to(db::api::event_get);
    })
}

fn main() {
    let addr = "0.0.0.0:8000";
    dotenv().ok();
    env_logger::init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be specify");
    gotham::start(addr, router(db::api::Repo::new(&db_url)));
}
