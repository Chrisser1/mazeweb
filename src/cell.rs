use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Wall = 1,
    Start = 2,
    End = 3,
    Path = 4,
    Visited = 5,
    LookingAt = 6,
    Current = 7,
    Changeing = 8,
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::Wall => Cell::Empty,
            _ => Cell::Wall
        }
    }
}
