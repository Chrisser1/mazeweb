use crate::{cell::{Cell, CellType}, maze::{Maze, MazeChange}, utils};

use super::generator::MazeGenerator;

pub struct AdlousBroder {
    total_cells: usize,
    visited_cells: usize,
}

impl MazeGenerator for AdlousBroder {
    fn generate_maze_steps(&mut self, original_maze: &Maze) -> Option<Vec<Vec<MazeChange>>> {
        let mut steps = Vec::new();
        let mut maze = original_maze.clone();

        // Initialize total cells and visited cells
        self.total_cells = maze.get_cells().len();
        self.visited_cells = 0;

        // Set all cells to be walls
        let walling_step: Vec<MazeChange> = self.add_all_walls(&mut maze);
        if !walling_step.is_empty() {
            steps.push(walling_step);
        }

        let (mut row, mut col) = utils::choose_random_cell(maze.height(), maze.width());

        let mut first_step: Vec<MazeChange> = Vec::new();
        let mut first = maze.get_cell(row, col);
        first.set_type(CellType::Current);
        self.mark_cell(&mut maze, row, col, first, &mut first_step);
        steps.push(first_step);
        self.visited_cells += 1;

        while self.total_cells > self.visited_cells {
            let (next_row, next_col) = utils::pick_random_neighbor(row, col, maze.width(), maze.height());

            let mut step = Vec::new();

            // mark old as visited
            let mut old = maze.get_cell(row, col);
            old.set_type(CellType::Visited);
            self.mark_cell(&mut maze, row, col, old, &mut step);

            // If it's not yet visited, carve the path and mark as visited
            let neighbor = maze.get_cell(next_row, next_col);
            if neighbor.get_type() != CellType::Visited {
                self.remove_wall_between(&mut maze, row, col, next_row, next_col, &mut step);
                self.visited_cells += 1;
            }

            // Mark the next cell as current
            let mut next = maze.get_cell(next_row, next_col);
            next.set_type(CellType::Current);
            self.mark_cell(&mut maze, next_row, next_col, next, &mut step);

            // push the step to the steps vector
            steps.push(step);


            // Move to the next cell
            row = next_row;
            col = next_col;
        }

        // Remove all visited cells and set them to default
        let step = self.set_all_visited_to_default(&mut maze);
        steps.push(step);

        Some(steps)
    }
}

impl AdlousBroder {
    pub fn new() -> Self {
        AdlousBroder {
            total_cells: 0,
            visited_cells: 0,
        }
    }
}
