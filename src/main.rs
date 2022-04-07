use limiter::Cooldown;
use options::OPTIONS as opts;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use serde::Serialize;
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use std::{sync::Mutex, thread};

#[macro_use]
extern crate rocket;
mod canvas;
mod cors;
mod limiter;
mod options;
mod state;

#[derive(Serialize)]
pub struct OptionsDto {
    width: usize,
    height: usize,
    refresh_period_miliseconds: u64,
    cooldown_duration_seconds: i64,
}

impl From<options::Options> for OptionsDto {
    fn from(opts_var: options::Options) -> Self {
        Self {
            width: opts_var.width,
            height: opts_var.height,
            refresh_period_miliseconds: opts_var.refresh_period_miliseconds,
            cooldown_duration_seconds: opts_var.cooldown_duration_seconds,
        }
    }
}

#[derive(Deserialize)]
pub struct ColorizeRequest {
    width: usize,
    height: usize,
    color: u8,
}

enum Operation {
    Colorize(ColorizeRequest),
    Publish,
}

#[catch(default)]
fn default_catcher() {}

#[get("/")]
fn get_canvas<'c>(state_factory: &State<state::StateFactory>) -> canvas::Canvas {
    state_factory.create().get()
}

#[get("/colors")]
fn get_colors() -> &'static str {
    opts.colors
}

#[get("/options")]
fn get_options() -> Json<OptionsDto> {
    Json(opts.into())
}

#[post("/", data = "<colorize>")]
fn colorize(
    colorize: Json<ColorizeRequest>,
    send_op: &State<Mutex<Sender<Operation>>>,
    _cooldown: Cooldown,
) {
    send_op
        .lock()
        .unwrap()
        .send(Operation::Colorize(colorize.into_inner()))
        .unwrap();
}

#[launch]
fn rocket() -> _ {
    let (mut state_writer, state_factory) = state::new();

    let (send, recv) = channel::<Operation>();
    thread::spawn(move || {
        while let Ok(op) = recv.recv() {
            match op {
                Operation::Colorize(c) => state_writer.colorize(c.width, c.height, c.color),
                Operation::Publish => state_writer.publish(),
            }
        }
    });

    let timer_send_handle = send.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(opts.refresh_period_miliseconds));
        timer_send_handle.send(Operation::Publish).unwrap();
    });

    rocket::build()
        .register("/", catchers![default_catcher])
        .manage(state_factory)
        .manage(Mutex::new(send))
        .manage(limiter::Limiter::new())
        .attach(cors::cors_fairing())
        .mount("/", routes![get_canvas, colorize, get_colors, get_options])
}
