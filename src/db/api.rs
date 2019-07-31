use diesel::prelude::*;
use futures::Future;
use gotham::handler::{HandlerError, IntoHandlerError};

use crate::api::utils::not_found;
use crate::db::modles::{Event, NewEvent, PathExtractor, UpdateEventStatus};
use crate::db::{schema, Repo};

pub fn create_event(
    repo: Repo,
    event: NewEvent,
) -> impl Future<Item = Event, Error = HandlerError> {
    repo.run(move |conn| {
        diesel::insert_into(schema::events::table)
            .values(&event)
            .get_result::<Event>(&conn)
    })
    .map_err(|e| e.into_handler_error())
}

pub fn update_event(
    repo: Repo,
    event: UpdateEventStatus,
) -> impl Future<Item = Event, Error = HandlerError> {
    use schema::events::dsl::*;
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
}

pub fn list_event(repo: Repo) -> impl Future<Item = Vec<Event>, Error = HandlerError> {
    use schema::events::dsl::*;
    repo.run(move |conn| events.load::<Event>(&conn))
        .map_err(|e| e.into_handler_error())
}

pub fn get_event(
    repo: Repo,
    extractor: PathExtractor,
) -> impl Future<Item = Event, Error = HandlerError> {
    use schema::events::dsl::*;

    repo.run(move |conn| {
        events
            .find(extractor.id)
            .get_result::<Event>(&conn)
            .map_err(|e| e.into_handler_error())
    })
    .map_err(|e| e.into_handler_error())
}

pub fn delete_event(
    repo: Repo,
    extractor: PathExtractor,
) -> impl Future<Item = usize, Error = HandlerError> {
    use schema::events::dsl::*;

    repo.run(move |conn| diesel::delete(events.find(extractor.id)).execute(&conn))
        .map_err(|e| e.into_handler_error())
}
