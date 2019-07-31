mod events;
pub mod utils;

use gotham::handler::assets::FileOptions;
use gotham::pipeline::{new_pipeline, single::single_pipeline};
use gotham::router::{builder::*, Router};
use gotham_middleware_diesel::DieselMiddleware;

use crate::db;

pub fn router(repo: db::Repo) -> Router {
    let (chain, pipeline) =
        single_pipeline(new_pipeline().add(DieselMiddleware::new(repo)).build());

    build_router(chain, pipeline, |route| {
        // routing index page and static files
        route.get("/").to_file("src/static/index.html");
        route.get("/favicon.ico").to_file("src/static/favicon.ico");
        route.get("/static/*").to_dir(
            FileOptions::new(&"src/static")
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build(),
        );
        route.scope("/api", |route| {
            route.scope("/events", |route| {
                route.get("/").to(self::events::get);
                route.post("/").to(self::events::post);
                route.put("/").to(self::events::put);
                route
                    .get("/:id")
                    .with_path_extractor::<db::PathExtractor>()
                    .to(self::events::show);
                route
                    .delete("/:id")
                    .with_path_extractor::<db::PathExtractor>()
                    .to(self::events::delete);
            });
        });
    })
}
