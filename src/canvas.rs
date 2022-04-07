use std::io::Cursor;

use rocket::{http::ContentType, response::Responder, Response};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

pub const COLORS: &'static str = include_str!("../colors.json");

pub type Color = u8;

#[derive(Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: [Color; WIDTH * HEIGHT],
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            canvas: [Color::default(); WIDTH * HEIGHT],
        }
    }
}

impl<'r> Responder<'r, 'static> for Canvas {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .sized_body(self.width * self.height, Cursor::new(self.canvas))
            .raw_header("x-width", self.width.to_string())
            .raw_header("x-height", self.height.to_string())
            .header(ContentType::new("application", "octet-stream"))
            .ok()
    }
}
