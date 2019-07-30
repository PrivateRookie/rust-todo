#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate log;

use dotenv::dotenv;
use gotham::handler::assets::FileOptions;
use gotham::pipeline::{new_pipeline, single::single_pipeline};
use gotham::router::{builder::*, Router};
use gotham_middleware_diesel::DieselMiddleware;
use std::env;

mod db;

fn router(repo: db::Repo) -> Router {
    let (chain, pipeline) =
        single_pipeline(new_pipeline().add(DieselMiddleware::new(repo)).build());

    build_router(chain, pipeline, |route| {
        route.get("/").to_file("src/static/index.html");
        route.get("/favicon.ico").to_file("src/static/favicon.ico");
        route.get("/static/*").to_dir(
            FileOptions::new(&"src/static")
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build(),
        );
        route.scope("/api", |route| {
            route.get("/events").to(db::api::event_list);
            route.post("/events").to(db::api::event_post);
            route.put("/events").to(db::api::update_status);
            route
                .get("/events/:id")
                .with_path_extractor::<db::PathExtractor>()
                .to(db::api::event_get);
            route
                .delete("/events/:id")
                .with_path_extractor::<db::PathExtractor>()
                .to(db::api::event_delete);
        });
    })
}

fn main() {
    let addr = "0.0.0.0:8000";
    dotenv().ok();
    env_logger::init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be specify");
    gotham::start(addr, router(db::Repo::new(&db_url)));
}
