use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use std::{sync::Mutex, thread};

#[macro_use]
extern crate rocket;
mod canvas;
mod state;

#[derive(Deserialize)]
pub struct Colorize {
    width: usize,
    height: usize,
    color: canvas::Color,
}

enum Operation {
    Colorize(Colorize),
    Publish,
}

#[get("/")]
fn get_canvas<'c>(state_factory: &State<state::StateFactory>) -> canvas::Canvas {
    state_factory.create().get()
}

#[post("/", data = "<colorize>")]
fn colorize(colorize: Json<Colorize>, send_op: &State<Mutex<Sender<Operation>>>) {
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
        thread::sleep(Duration::from_millis(1000));
        timer_send_handle.send(Operation::Publish).unwrap();
    });

    rocket::build()
        .manage(state_factory)
        .manage(Mutex::new(send))
        .mount("/", routes![get_canvas, colorize])
}
