use futures::{Future, Stream};
use gotham::handler::{HandlerError, IntoHandlerError};
use gotham::state::{FromState, State};
use hyper::{Body, StatusCode};
use log::warn;
use std::str::from_utf8;

pub fn bad_request<E>(e: E) -> HandlerError
where
    E: std::error::Error + Send + 'static,
{
    warn!("error occur: {}", e);
    e.into_handler_error().with_status(StatusCode::BAD_REQUEST)
}

pub fn not_found<E>(e: E) -> HandlerError
where
    E: std::error::Error + Send + 'static,
{
    warn!("error occur: {}", e);
    e.into_handler_error().with_status(StatusCode::NOT_FOUND)
}

pub fn extract_json<T>(state: &mut State) -> impl Future<Item = T, Error = HandlerError>
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
