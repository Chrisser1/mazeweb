use wasm_bindgen::prelude::*;

use crate::{cell::Cell, generators::aldous_broder::AdlousBroder, maze::{Maze, MazeChange}};

pub trait MazeGenerator {
    fn generate_maze_steps(&mut self, maze: &Maze) -> Option<Vec<Vec<MazeChange>>>;

    /// Default method: mark a cell with any new state and record the change
    fn mark_cell(
        &self,
        maze: &mut Maze,
        row: u32,
        col: u32,
        new_state: Cell,
        step: &mut Vec<MazeChange>,
    ) {
        let old = maze.get_cell(row, col);
        if old != new_state {
            maze.set_cell(row, col, new_state);
            step.push(MazeChange {
                row,
                col,
                old,
                new: new_state,
            });
        }
    }

    /// Default method: remove the wall between two cells
    fn remove_wall_between(
        &self,
        maze: &mut Maze,
        row1: u32,
        col1: u32,
        row2: u32,
        col2: u32,
        step: &mut Vec<MazeChange>,
    ) {
        let wall_row = (row1 + row2) / 2;
        let wall_col = (col1 + col2) / 2;

        let old = maze.get_cell(wall_row, wall_col);
        if old != Cell::Visited {
            maze.set_cell(wall_row, wall_col, Cell::Visited);
            step.push(MazeChange {
                row: wall_row,
                col: wall_col,
                old: old,
                new: Cell::Visited,
            });
        }
    }

    /// Default method: set all cells to walls, returning the changes
    fn make_all_into(&self, maze: &mut Maze, cell: Cell) -> Vec<MazeChange> {
        let mut changes = Vec::new();

        for row in 0..maze.height() {
            for col in 0..maze.width() {
                let old = maze.get_cell(row, col);
                if old != cell {
                    maze.set_cell(row, col, cell);
                    changes.push(MazeChange {
                        row,
                        col,
                        old,
                        new: cell,
                    });
                }
            }
        }

        changes
    }

    fn set_all_visited_to_empty(&self, maze: &mut Maze) -> Vec<MazeChange> {
        let mut changes = Vec::new();

        for row in 0..maze.height() {
            for col in 0..maze.width() {
                let old = maze.get_cell(row, col);
                if old == Cell::Visited {
                    maze.set_cell(row, col, Cell::Empty);
                    changes.push(MazeChange {
                        row,
                        col,
                        old,
                        new: Cell::Empty,
                    });
                }
            }
        }
        changes
    }
}

#[wasm_bindgen]
pub struct MazeBuilder {
    generator: Box<dyn MazeGenerator>,
    steps: Vec<Vec<MazeChange>>,
    current_step: usize,
}

#[wasm_bindgen]
impl MazeBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MazeBuilder {
        let generator = Box::new(AdlousBroder::new());
        MazeBuilder {
            generator,
            steps: vec![vec![]],
            current_step: 0,
        }
    }

    #[wasm_bindgen(js_name = "withGenerator")]
    pub fn with_generator(name: &str) -> MazeBuilder {
        let generator: Box<dyn MazeGenerator> = match name {
            "aldous_broder" => Box::new(AdlousBroder::new()),
            "recursive_division" => Box::new(crate::generators::recursive_division::RecursiveDivision::new()),
            _ => Box::new(AdlousBroder::new()), // fallback
        };

        MazeBuilder {
            generator,
            steps: vec![vec![]],
            current_step: 0,
        }
    }

    /// Generate all steps for building a maze and store them in steps.
    /// Also set the current step to zero.
    pub fn generate_all(&mut self, maze: &Maze) {
        self.steps = self.generator.generate_maze_steps(maze).unwrap_throw();
        self.current_step = 0;
    }

    pub fn step_forward(&mut self, maze: &mut Maze) -> bool {
        if self.current_step < self.steps.len() {
            // Replay saved step
            for change in &self.steps[self.current_step] {
                maze.set_cell(change.row, change.col, change.new);
            }
            self.current_step += 1;
            return true;
        }

        // If there is no steps left return false to indicate that
        false
    }

    pub fn step_backward(&mut self, maze: &mut Maze) -> bool {
        if self.current_step == 0 {
            return false;
        }

        self.current_step -= 1;
        for change in &self.steps[self.current_step] {
            maze.set_cell(change.row, change.col, change.old);
        }

        true
    }

    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }

    pub fn current_step(&self) -> usize {
        self.current_step
    }

    pub fn reset(&mut self) {
        self.current_step = 0;
    }

    pub fn step_to(&mut self, target: usize, maze: &mut Maze) {
        self.reset();
        for _ in 0..=target {
            self.step_forward(maze);
        }
    }
}
