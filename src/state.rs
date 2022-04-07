use crate::canvas::{Canvas, Color};
use left_right::{Absorb, ReadHandle, ReadHandleFactory, WriteHandle};

struct ColorizeOp {
    pub pos: (usize, usize),
    pub color: Color,
}

impl Absorb<ColorizeOp> for Canvas {
    fn absorb_first(&mut self, operation: &mut ColorizeOp, _other: &Self) {
        self.colorize(operation.pos, operation.color);
    }

    fn sync_with(&mut self, first: &Self) {
        self.canvas.copy_from_slice(&first.canvas);
    }
}

pub struct StateWriter(WriteHandle<Canvas, ColorizeOp>);

impl StateWriter {
    pub fn colorize(&mut self, width: usize, height: usize, color: Color) {
        let op = ColorizeOp {
            pos: (width, height),
            color,
        };

        self.0.append(op);
    }

    pub fn publish(&mut self) {
        self.0.publish();
    }
}

pub struct State(ReadHandle<Canvas>);
pub struct StateFactory(ReadHandleFactory<Canvas>);

impl State {
    pub fn get(&self) -> Canvas {
        self.0.enter().map(|guard| (*guard).clone()).unwrap() //TODO: Cloning is not good here
    }
}

impl StateFactory {
    pub fn create(&self) -> State {
        State(self.0.handle())
    }
}

pub fn new() -> (StateWriter, StateFactory) {
    let (write_handle, read_handle) = left_right::new::<Canvas, ColorizeOp>();
    (
        StateWriter(write_handle),
        StateFactory(read_handle.factory()),
    )
}
