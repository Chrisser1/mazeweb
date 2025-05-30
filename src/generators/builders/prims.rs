use js_sys::Math::random;

use crate::{cell::{Cell, CellType}, generators::generator::MazeGenerator, maze::{Maze, MazeChange}, utils};


pub struct Prims;

impl Prims {
    pub fn new() -> Self {
        Prims
    }
}

impl MazeGenerator for Prims {
    fn generate_maze_steps(&mut self, original_maze: &Maze) -> Option<Vec<Vec<MazeChange>>> {
        let mut steps = Vec::new();
        let mut maze = original_maze.clone();
        let h = maze.height() as usize;
        let w = maze.width() as usize;

        // Start with all walls present
        let walling_step: Vec<MazeChange> = self.add_all_walls(&mut maze);
        if !walling_step.is_empty() {
            steps.push(walling_step);
        }

        // Pick a random starting cell
        let (start_row, start_col) = utils::choose_random_cell(h as u32, w as u32);
        let mut first_step: Vec<MazeChange> = Vec::new();
        let mut first_cell = maze.get_cell(start_row, start_col);
        first_cell.set_type(CellType::Visited);
        self.mark_cell(&mut maze, start_row, start_col, first_cell, &mut first_step);
        steps.push(first_step);

        // Initialize a wall list from the starting cell
        let mut wall_list = Vec::new();
        for &(dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nr = start_row as i32 + dr;
            let nc = start_col as i32 + dc;
            if nr < h as i32 && nc < w as i32 {
                wall_list.push((start_row, start_col, dr, dc));
            }
        }

        while !wall_list.is_empty() {
            // Pick a random wall from the list
            let i = (random() * wall_list.len() as f64).floor() as usize;
            let (r, c, dr, dc) = wall_list.swap_remove(i);
            let nr = (r as i32 + dr) as u32;
            let nc = (c as i32 + dc) as u32;

            // Check if the neighboring cell is within bounds and not visited
            if nr >= h as u32 || nc >= w as u32 {
                continue;
            }

            // count how many of the two cells are already in the maze
            let cell1 = maze.get_cell(r, c).get_type() != CellType::Default;
            let cell2 = maze.get_cell(nr, nc).get_type() != CellType::Default;

            // If exactly one of the cells is in the maze, carve the wall
            if cell1 ^ cell2 {
                // Mark as current cell
                let mut current_cell = maze.get_cell(r, c);
                current_cell.set_type(CellType::Current);
                let mut current_step = Vec::new();
                self.mark_cell(&mut maze, r, c, current_cell, &mut current_step);
                steps.push(current_step);

                // carve passage
                let mut step = Vec::new();
                self.remove_wall_between(&mut maze, r, c, nr, nc, &mut step);
                // mark the newly reached cell
                let (vr, vc) = if !cell1 { (r, c) } else { (nr, nc) };
                self.mark_cell(&mut maze, vr, vc, Cell::new(CellType::Visited), &mut step);

                // add that cell's neighboring walls
                for &(adr, adc) in &[( -1,  0), ( 1,  0), ( 0, -1), ( 0,  1)] {
                    let ar = (vr as i32 + adr) as u32;
                    let ac = (vc as i32 + adc) as u32;
                    if ar < h as u32 && ac < w as u32 {
                        wall_list.push((vr, vc, adr, adc));
                    }
                }

                // Remove tho current cell marking
                let mut prev = maze.get_cell(r, c);
                prev.set_type(CellType::Visited);
                self.mark_cell(&mut maze, r, c, prev, &mut step);

                steps.push(step);
            }
            // If both cells are already in the maze, do nothing
        }

        Some(steps)
    }
}
