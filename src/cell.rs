use wasm_bindgen::prelude::*;

pub const WALL_N: u8 = 0b00000001;
pub const WALL_E: u8 = 0b00000010;
pub const WALL_S: u8 = 0b00000100;
pub const WALL_W: u8 = 0b00001000;
pub const WALL_MASK: u8 = 0b00001111;
pub const TYPE_MASK: u8 = 0b11110000;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    value: u8,
}

#[wasm_bindgen]
impl Cell {
    /// Get the cell type
    pub fn get_type(&self) -> CellType {
        match self.value & TYPE_MASK {
            0x00 => CellType::Default,
            0x10 => CellType::Start,
            0x20 => CellType::End,
            0x30 => CellType::Path,
            0x40 => CellType::Visited,
            0x50 => CellType::LookingAt,
            0x60 => CellType::Current,
            0x70 => CellType::Changing,
            _ => CellType::Default, // fallback
        }
    }



}

impl Cell {
    /// Create a new Cell with a specific type and no walls
    pub fn new(cell_type: CellType) -> Cell {
        Cell {
            value: cell_type as u8,
        }
    }

    /// Get the raw u8 representation
    pub fn raw(&self) -> u8 {
        self.value
    }

    /// Set the raw value (careful!)
    pub fn set_raw(&mut self, val: u8) {
        self.value = val;
    }

    /// Set the cell type
    pub fn set_type(&mut self, cell_type: CellType) {
        self.value = (self.value & WALL_MASK) | (cell_type as u8);
    }

    /// Toggle between Wall and Empty
    pub fn toggle(&mut self) {
        let new_type = match self.get_type() {
            CellType::Default => CellType::Changing,
            _ => CellType::Default,
        };
        self.set_type(new_type);
    }


    /// Check if a wall exists
    pub fn has_wall(&self, wall: u8) -> bool {
        self.value & wall != 0
    }

    /// Add a wall
    pub fn add_wall(&mut self, wall: u8) {
        self.value |= wall;
    }

    /// Remove a wall
    pub fn remove_wall(&mut self, wall: u8) {
        self.value &= !wall;
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Default   = 0,   // 0 << 4
    Start     = 16,  // 1 << 4
    End       = 32,  // 2 << 4
    Path      = 48,  // 3 << 4
    Visited   = 64,  // 4 << 4
    LookingAt = 80,  // 5 << 4
    Current   = 96,  // 6 << 4
    Changing  = 112, // 7 << 4
}
