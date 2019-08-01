use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::create_empty_response;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{request_id, FromState, State};
use hyper::header::HeaderMap;
use hyper::StatusCode;
use log::{info, warn};

use std::io;

pub struct AuthMiddleWare {
    username: String,
    password: String,
}

impl AuthMiddleWare {
    pub fn new() -> AuthMiddleWare {
        AuthMiddleWare {
            username: "admin".to_string(),
            password: "admin".to_string(),
        }
    }
}

impl NewMiddleware for AuthMiddleWare {
    type Instance = AuthMiddleWare;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(AuthMiddleWare {
            username: self.username.clone(),
            password: self.password.clone(),
        })
    }
}

impl Middleware for AuthMiddleWare {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        let header = HeaderMap::borrow_from(&state);

        match (header.get("password"), header.get("username")) {
            (Some(username), Some(password)) => {
                if username.to_str().unwrap().to_string() == self.username
                    && password.to_str().unwrap().to_string() == self.password
                {
                    info!("{} auth successed", request_id(&state));
                    chain(state)
                } else {
                    abort_401(state)
                }
            }
            _ => abort_401(state),
        }
    }
}

fn abort_401(state: State) -> Box<HandlerFuture> {
    warn!("{} auth failed", request_id(&state));
    let resp = create_empty_response(&state, StatusCode::UNAUTHORIZED);
    Box::new(futures::future::ok((state, resp)))
}
