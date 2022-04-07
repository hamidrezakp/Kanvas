use rocket::http::Method;
use rocket_cors::{AllowedHeaders, Cors};

pub fn cors_fairing() -> Cors {
    rocket_cors::CorsOptions {
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::All,
        ..Default::default()
    }
    .to_cors()
    .unwrap() //TODO propagate error to upper layers
}
