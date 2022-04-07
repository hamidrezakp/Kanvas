use crate::options::OPTIONS as opts;
use rocket::{http::ContentType, response::Responder, Response};
use std::io::Cursor;

pub type Color = u8;

#[derive(Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: [Color; opts.width * opts.height],
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            width: opts.width,
            height: opts.height,
            canvas: [Color::default(); opts.width * opts.height],
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
