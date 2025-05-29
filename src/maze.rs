use std::{convert::TryInto, fmt};
use wasm_bindgen::prelude::*;

use crate::{cell::{Cell, CellType}, utils};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Maze {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Maze {
    pub fn new(width: u32, height: u32) -> Maze {
        utils::set_panic_hook();

        Maze {
            width,
            height,
            cells: vec![Cell::new(CellType::Default); (width * height).try_into().unwrap()],
        }
    }

    /// Toggle cells to be wall or empty
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    /// Set the width and height of the maze.
    ///
    /// Resets the cells to `Cell::Empty`.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height)
            .map(|_i| Cell::new(CellType::Default))
            .collect();
    }

    /// Set the height of the maze.
    ///
    /// Resets the cells to `Cell::Empty`.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height)
            .map(|_i| Cell::new(CellType::Default))
            .collect();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        assert!(row < self.height, "Row out of bounds: {} >= {}", row, self.height);
        assert!(col < self.width, "Column out of bounds: {} >= {}", col, self.width);
        (row * self.width + col) as usize
    }
}

impl Maze {
    /// Get the cell state for the full maze
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be a wall in the maze by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)], cell: Cell) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = cell;
        }
    }

    pub fn set_cell(&mut self, row: u32, col: u32, cell: Cell) {
        let idx = self.get_index(row, col);
        self.cells[idx] = cell;
    }

    pub fn get_cell(&self, row: u32, col: u32) -> Cell {
        let idx = self.get_index(row, col);
        self.cells[idx]
    }

    pub fn get_cell_mut(&mut self, row: u32, col: u32) -> &mut Cell {
        let idx = self.get_index(row, col);
        &mut self.cells[idx]
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell.get_type() == CellType::Default { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct MazeChange {
    pub row: u32,
    pub col: u32,
    pub old: Cell,
    pub new: Cell,
}
