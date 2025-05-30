use wasm_bindgen::prelude::*;

use crate::{
    cell::{
        Cell,
        CellType,
        TYPE_MASK, WALL_E,
        WALL_MASK, WALL_N,
        WALL_S, WALL_W
    },
    generators::builders::{
        AdlousBroder,
        Kruskals,
        Prims,
        RecursiveDivision
    },
    maze::{Maze, MazeChange}
};


pub trait MazeGenerator {
    fn generate_maze_steps(&mut self, maze: &Maze) -> Option<Vec<Vec<MazeChange>>>;

    /// Default method: mark a cell with any new state and record the change
    fn mark_cell(
        &self,
        maze: &mut Maze,
        row: u32,
        col: u32,
        mut new_state: Cell,
        step: &mut Vec<MazeChange>,
    ) {
        let old = maze.get_cell(row, col);

        // Carry forward walls from old cell to the new one
        let walls = old.raw() & WALL_MASK;
        new_state.set_raw((new_state.raw() & TYPE_MASK) | walls);

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
        let old_c1 = maze.get_cell(row1, col1);
        let old_c2 = maze.get_cell(row2, col2);

        let (wall_from_c1, wall_from_c2) = match (row2 as i32 - row1 as i32, col2 as i32 - col1 as i32) {
            (-1, 0) => (WALL_N, WALL_S),
            (1, 0) => (WALL_S, WALL_N),
            (0, -1) => (WALL_W, WALL_E),
            (0, 1) => (WALL_E, WALL_W),
            _ => return,
        };

        let mut c1 = old_c1;
        let mut c2 = old_c2;

        c1.remove_wall(wall_from_c1);
        c2.remove_wall(wall_from_c2);

        maze.set_cell(row1, col1, c1);
        maze.set_cell(row2, col2, c2);

        step.push(MazeChange { row: row1, col: col1, old: old_c1, new: c1 });
        step.push(MazeChange { row: row2, col: col2, old: old_c2, new: c2 });
    }

    /// Add the wall between two adjacent cells and record both changes.
    fn add_wall_between(
        &self,
        maze: &mut Maze,
        row1: u32,
        col1: u32,
        row2: u32,
        col2: u32,
        step: &mut Vec<MazeChange>,
    ) {
        let old1 = maze.get_cell(row1, col1);
        let old2 = maze.get_cell(row2, col2);
        let (w1, w2) = match (row2 as i32 - row1 as i32, col2 as i32 - col1 as i32) {
            (-1, 0) => (WALL_N, WALL_S),
            (1, 0) => (WALL_S, WALL_N),
            (0, -1) => (WALL_W, WALL_E),
            (0, 1) => (WALL_E, WALL_W),
            _ => return,
        };
        let mut c1 = old1;
        let mut c2 = old2;
        c1.add_wall(w1);
        c2.add_wall(w2);
        maze.set_cell(row1, col1, c1);
        maze.set_cell(row2, col2, c2);
        step.push(MazeChange { row: row1, col: col1, old: old1, new: c1 });
        step.push(MazeChange { row: row2, col: col2, old: old2, new: c2 });
    }

    /// Remove a wall in the specified direction (dr, dc) from the given cell
    fn remove_wall_dir(
        &self,
        maze: &mut Maze,
        row: u32,
        col: u32,
        dr: i32,
        dc: i32,
        step: &mut Vec<MazeChange>,
    ) {
        let nr = (row as i32 + dr) as u32;
        let nc = (col as i32 + dc) as u32;
        self.remove_wall_between(maze, row, col, nr, nc, step);
    }

    /// Add a wall in the specified direction (dr, dc) from the given cell
    fn add_wall_dir(
        &self,
        maze: &mut Maze,
        row: u32,
        col: u32,
        dr: i32,
        dc: i32,
        step: &mut Vec<MazeChange>,
    ) {
        let nr = (row as i32 + dr) as u32;
        let nc = (col as i32 + dc) as u32;
        self.add_wall_between(maze, row, col, nr, nc, step);
    }

    /// Default method: make all cells into a specific type
    fn make_all_into(&self, maze: &mut Maze, cell_type: CellType) -> Vec<MazeChange> {
        let mut changes = Vec::new();

        for row in 0..maze.height() {
            for col in 0..maze.width() {
                let current = maze.get_cell(row, col);
                if current.get_type() != cell_type {
                    let walls = current.raw() & WALL_MASK;
                    let mut new_cell = Cell::new(cell_type);
                    new_cell.set_raw((new_cell.raw() & TYPE_MASK) | walls);

                    maze.set_cell(row, col, new_cell);
                    changes.push(MazeChange {
                        row,
                        col,
                        old: current,
                        new: new_cell,
                    });
                }
            }
        }

        changes
    }

    /// Add all 4 walls to every cell
    fn add_all_walls(&self, maze: &mut Maze) -> Vec<MazeChange> {
        let mut changes = Vec::new();

        for row in 0..maze.height() {
            for col in 0..maze.width() {
                let mut cell = maze.get_cell(row, col);
                let old = cell;
                cell.add_wall(WALL_N | WALL_E | WALL_S | WALL_W);
                maze.set_cell(row, col, cell);
                changes.push(MazeChange {
                    row,
                    col,
                    old,
                    new: cell,
                });
            }
        }

        changes
    }

    /// Remove all walls from every cell
    fn remove_all_walls(&self, maze: &mut Maze) -> Vec<MazeChange> {
        let mut changes = Vec::new();

        for row in 0..maze.height() {
            for col in 0..maze.width() {
                let mut cell = maze.get_cell(row, col);
                let old = cell;
                cell.remove_wall(WALL_N | WALL_E | WALL_S | WALL_W);
                maze.set_cell(row, col, cell);
                changes.push(MazeChange {
                    row,
                    col,
                    old,
                    new: cell,
                });
            }
        }

        changes
    }

    // Set all visited cells to default state
    fn set_all_visited_to_default(&self, maze: &mut Maze) -> Vec<MazeChange> {
        let mut changes = Vec::new();

        for row in 0..maze.height() {
            for col in 0..maze.width() {
                let old = maze.get_cell(row, col);
                if old.get_type() == CellType::Visited {
                    let walls = old.raw() & WALL_MASK;
                    let mut new_cell = Cell::new(CellType::Default);
                    new_cell.set_raw((new_cell.raw() & TYPE_MASK) | walls);

                    maze.set_cell(row, col, new_cell);
                    changes.push(MazeChange {
                        row,
                        col,
                        old,
                        new: new_cell,
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
            "recursive_division" => Box::new(RecursiveDivision::new()),
            "kruskals" => Box::new(Kruskals::new()),
            "prims" => Box::new(Prims::new()),
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
