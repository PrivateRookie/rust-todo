#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate log;

mod api;
mod db;

use clap::{App, Arg};
use dotenv::dotenv;
use std::env;

use crate::api::auth::AuthMiddleWare;
use crate::api::router;

fn main() {
    let app = App::new("api")
        .version("0.1")
        .author("PrivateRookie")
        .about("gotham pratics demo")
        .arg(
            Arg::with_name("host")
                .long("host")
                .help("Listening Host")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .help("Listening Port")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("auth")
                .long("auth")
                .help("Enable Auth with http header, format: username:passwd")
                .takes_value(true)
                .validator(AuthMiddleWare::validate),
        );

    let matches = app.get_matches_safe().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1)
    });

    let addr = format!(
        "{}:{}",
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap()
    );
    dotenv().ok();
    env_logger::init();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be specify");
    let repo = db::Repo::new(&db_url);
    let auth = matches.value_of("auth");
    let userlist = vec!["admin:admin".to_owned(), "xd:super".to_owned()];
    gotham::start(addr, router(repo, auth, userlist));
}
