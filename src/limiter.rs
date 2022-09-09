use crate::options::OPTIONS as opts;
use chrono::{prelude::*, Duration};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::Request;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;

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
        if let Some(limiter) = req.rocket().state::<Limiter>() {
            if let Ok(mut map) = limiter.0.lock() {
                if let Some(remote_addr) = req.remote() {
                    let now = Utc::now().naive_utc();
                    match map.get(&remote_addr) {
                        Some(due_time) if now < *due_time => {
                            Outcome::Failure((Status::TooManyRequests, ()))
                        }
                        _ => {
                            map.insert(
                                remote_addr,
                                now + Duration::seconds(opts.cooldown_duration_seconds),
                            );
                            Outcome::Success(Cooldown)
                        }
                    }
                } else {
                    Outcome::Failure((Status::BadRequest, ()))
                }
            } else {
                Outcome::Failure((Status::InternalServerError, ()))
            }
        } else {
            Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}
