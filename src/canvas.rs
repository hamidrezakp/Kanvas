use std::io::Cursor;

use rocket::{http::ContentType, response::Responder, serde::Deserialize, Response};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

#[derive(Clone, Copy, Deserialize)]
pub enum Color {
    Black = 0,
    White = 1,
}

impl Default for Color {
    fn default() -> Self {
        Self::Black
    }
}

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
        let canvas: Vec<u8> = self.canvas.iter().map(|c| *c as u8).collect();
        Response::build()
            .sized_body(self.width * self.height, Cursor::new(canvas))
            .raw_header("x-width", self.width.to_string())
            .raw_header("x-height", self.height.to_string())
            .header(ContentType::new("application", "octet-stream"))
            .ok()
    }
}
