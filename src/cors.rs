use rocket::http::Method;
use rocket_cors::{Cors};

pub fn cors_fairing() -> Cors {
    rocket_cors::CorsOptions {
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        ..Default::default()
    }
    .to_cors()
    .unwrap() //TODO propagate error to upper layers
}
