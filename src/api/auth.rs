use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::create_response;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{FromState, State};
use hyper::header::HeaderMap;
use hyper::StatusCode;

use std::io;

#[derive(Clone)]
pub struct AuthMiddleWare {
    username: String,
    password: String,
    enable: bool,
}

impl AuthMiddleWare {
    pub fn new(auth: Option<&str>) -> AuthMiddleWare {
        match auth {
            Some(auth) => {
                let parts = auth.splitn(2, ":").collect::<Vec<&str>>();
                AuthMiddleWare {
                    username: parts[0].to_owned(),
                    password: parts[1].to_owned(),
                    enable: true,
                }
            }
            None => AuthMiddleWare {
                username: String::new(),
                password: String::new(),
                enable: false,
            },
        }
    }

    pub fn validate(auth: String) -> Result<(), String> {
        // validate if auth argument a valid "username:passwod" format
        match auth.is_empty() {
            true => Ok(()),
            false => {
                let part = auth.split(":").collect::<Vec<&str>>();
                if part.len() == 2 {
                    Ok(())
                } else {
                    Err("invlid auth param format, please use: username:password".to_string())
                }
            }
        }
    }
}

impl NewMiddleware for AuthMiddleWare {
    type Instance = AuthMiddleWare;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl Middleware for AuthMiddleWare {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        match self.enable {
            true => {
                let header = HeaderMap::borrow_from(&state);

                match (header.get("password"), header.get("username")) {
                    (Some(username), Some(password)) => {
                        if username.to_str().unwrap().to_string() == self.username
                            && password.to_str().unwrap().to_string() == self.password
                        {
                            chain(state)
                        } else {
                            abort_401(state)
                        }
                    }
                    _ => abort_401(state),
                }
            }
            false => chain(state),
        }
    }
}

fn abort_401(state: State) -> Box<HandlerFuture> {
    let body: &'static [u8] = b"Unauthorized";
    let resp = create_response(&state, StatusCode::UNAUTHORIZED, mime::TEXT_PLAIN, body);
    Box::new(futures::future::ok((state, resp)))
}
