mod modles;
mod schema;

pub mod api;

use chrono::{NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use gotham::handler::{HandlerError, IntoHandlerError};
use hyper::StatusCode;
use log::warn;

pub use modles::PathExtractor;

pub type Repo = gotham_middleware_diesel::Repo<PgConnection>;

fn naivedate_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

fn bad_request<E>(e: E) -> HandlerError
where
    E: std::error::Error + Send + 'static,
{
    warn!("error occur: {}", e);
    e.into_handler_error().with_status(StatusCode::BAD_REQUEST)
}

fn not_found<E>(e: E) -> HandlerError
where
    E: std::error::Error + Send + 'static,
{
    warn!("error occur: {}", e);
    e.into_handler_error().with_status(StatusCode::NOT_FOUND)
}
