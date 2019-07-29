use diesel::pg::PgConnection;
use diesel::prelude::*;
use futures::{future, Future, Stream};
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use hyper::{Body, StatusCode};
use log::warn;
use serde_derive::Serialize;
use std::str::from_utf8;

use crate::db::modles::{Event, NewEvent, PathExtractor, UpdateEventStatus};
use crate::db::schema;

#[derive(Serialize)]
struct RowsUpdated {
    rows: usize,
}

pub type Repo = gotham_middleware_diesel::Repo<PgConnection>;

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

fn extract_json<T>(state: &mut State) -> impl Future<Item = T, Error = HandlerError>
where
    T: serde::de::DeserializeOwned,
{
    Body::take_from(state)
        .concat2()
        .map_err(bad_request)
        .and_then(|body| {
            let b = body.to_vec();
            from_utf8(&b)
                .map_err(bad_request)
                .and_then(|s| serde_json::from_str::<T>(s).map_err(bad_request))
        })
}

pub fn event_post(mut state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let f = extract_json::<NewEvent>(&mut state)
        .and_then(move |event| {
            repo.run(move |conn| {
                diesel::insert_into(schema::events::table)
                    .values(&event)
                    .execute(&conn)
            })
            .map_err(|e| e.into_handler_error())
        })
        .then(|result| match result {
            Ok(rows) => {
                let body = serde_json::to_string(&RowsUpdated { rows })
                    .expect("Failed to serialise to json");
                let res =
                    create_response(&state, StatusCode::CREATED, mime::APPLICATION_JSON, body);
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });
    Box::new(f)
}

pub fn update_status(mut state: State) -> Box<HandlerFuture> {
    use crate::db::schema::events::dsl::*;

    let repo = Repo::borrow_from(&state).clone();
    let f = extract_json::<UpdateEventStatus>(&mut state)
        .and_then(move |event| {
            repo.run(move |conn| {
                diesel::update(&event)
                    .set((
                        finished.eq(event.finished),
                        updated_at.eq(super::naivedate_now()),
                    ))
                    .get_result(&conn)
                    .map_err(|e| not_found(e))
            })
            .map_err(|e| e.into_handler_error())
        })
        .then(|result: Result<Event, HandlerError>| match result {
            Ok(event) => {
                let body = serde_json::to_string(&event).expect("Failed to serialize to json");
                let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });
    Box::new(f)
}

pub fn event_list(state: State) -> Box<HandlerFuture> {
    use crate::db::schema::events::dsl::*;

    let repo = Repo::borrow_from(&state).clone();
    let f = repo
        .run(move |conn| events.load::<Event>(&conn))
        .then(|result| match result {
            Ok(all_events) => {
                let body = serde_json::to_string(&all_events).expect("Failed to serialize events");
                let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });
    Box::new(f)
}

pub fn event_get(mut state: State) -> Box<HandlerFuture> {
    use crate::db::api::schema::events::dsl::*;

    let extrator = PathExtractor::take_from(&mut state);
    let repo = Repo::borrow_from(&state).clone();
    let f = repo
        .run(move |conn| {
            events
                .find(extrator.id)
                .get_result::<Event>(&conn)
                .map_err(|e| not_found(e))
        })
        .then(|result| match result {
            Ok(event) => {
                let body = serde_json::to_string(&event).expect("Failed to serialize event");
                let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });
    Box::new(f)
}
