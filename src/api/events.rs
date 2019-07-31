use futures::{future, Future};
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use hyper::StatusCode;

use crate::api::utils::extract_json;
use crate::db::modles::{Event, NewEvent, PathExtractor, UpdateEventStatus};
use crate::db::Repo;

use crate::db::api::{create_event, delete_event, get_event, list_event, update_event};

pub fn post(mut state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let f = extract_json::<NewEvent>(&mut state)
        .and_then(move |event| create_event(repo, event))
        .then(|result| match result {
            Ok(event) => {
                let body = serde_json::to_string(&event).expect("Failed to serialise to json");
                let res =
                    create_response(&state, StatusCode::CREATED, mime::APPLICATION_JSON, body);
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });
    Box::new(f)
}

pub fn put(mut state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let f = extract_json::<UpdateEventStatus>(&mut state)
        .and_then(move |event| update_event(repo, event))
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

pub fn get(state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let f = list_event(repo).then(|result| match result {
        Ok(all_events) => {
            let body = serde_json::to_string(&all_events).expect("Failed to serialize events");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
            future::ok((state, res))
        }
        Err(e) => future::err((state, e.into_handler_error())),
    });
    Box::new(f)
}

pub fn show(mut state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let extractor = PathExtractor::take_from(&mut state);
    let f = get_event(repo, extractor).then(|result| match result {
        Ok(event) => {
            let body = serde_json::to_string(&event).expect("Failed to serialize event");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);
            future::ok((state, res))
        }
        Err(e) => future::err((state, e.into_handler_error())),
    });
    Box::new(f)
}

pub fn delete(mut state: State) -> Box<HandlerFuture> {
    let repo = Repo::borrow_from(&state).clone();
    let extractor = PathExtractor::take_from(&mut state);
    let f = delete_event(repo, extractor).then(|result| match result {
        Ok(_) => {
            let res = create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                "".to_string(),
            );
            future::ok((state, res))
        }
        Err(e) => future::err((state, e.into_handler_error())),
    });
    Box::new(f)
}
