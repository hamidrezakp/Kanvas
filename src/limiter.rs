use chrono::{prelude::*, Duration};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::Request;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;

const COOLDOWN_DURATION_SECONDS: i64 = 10;

pub struct Limiter(Mutex<HashMap<SocketAddr, NaiveDateTime>>);

impl Limiter {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

pub struct Cooldown;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Cooldown {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.rocket().state::<Limiter>() {
            None => Outcome::Failure((Status::InternalServerError, ())),
            Some(limiter) => match limiter.0.lock() {
                Err(_) => Outcome::Failure((Status::InternalServerError, ())),
                Ok(mut map) => match req.remote() {
                    None => Outcome::Failure((Status::BadRequest, ())),
                    Some(remote_addr) => {
                        let now = Utc::now().naive_utc();
                        match map.get(&remote_addr) {
                            Some(due_time) if now < *due_time => {
                                Outcome::Failure((Status::TooManyRequests, ()))
                            }
                            _ => {
                                map.insert(
                                    remote_addr,
                                    now + Duration::seconds(COOLDOWN_DURATION_SECONDS),
                                );
                                Outcome::Success(Cooldown)
                            }
                        }
                    }
                },
            },
        }
    }
}
