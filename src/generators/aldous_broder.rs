use crate::{cell::Cell, maze::{Maze, MazeChange}, utils};

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
        self.total_cells = (((maze.height() as usize) - 1) / 2 + 1) * (((maze.width() as usize) - 1) / 2 + 1);
        self.visited_cells = 0;

        // Set all cells to be walls
        let walling_step: Vec<MazeChange> = self.make_all_into(&mut maze, Cell::Wall);
        if !walling_step.is_empty() {
            steps.push(walling_step);
        }

        let (mut row, mut col) = utils::choose_random_path_cell(maze.height(), maze.width());

        let mut first_step: Vec<MazeChange> = Vec::new();
        self.mark_cell(&mut maze, row, col, Cell::Current, &mut first_step);
        steps.push(first_step);
        self.visited_cells += 1;

        while self.total_cells > self.visited_cells {
            let (next_row, next_col) = utils::pick_random_neighbor(row, col, maze.width(), maze.height());

            // Mark the current cell as visited instead of current
            let mut step = Vec::new();
            self.mark_cell(&mut maze, row, col, Cell::Visited, &mut step);

            let neighbor = maze.get_cell(next_row, next_col);

            // If it's not yet visited, carve the path and mark as visited
            if neighbor != Cell::Visited {
                self.remove_wall_between(&mut maze, row, col, next_row, next_col, &mut step);
                self.visited_cells += 1;
            }

            // Mark the next cell as current
            self.mark_cell(&mut maze, next_row, next_col, Cell::Current, &mut step);
            steps.push(step);

            // Move to the next cell
            row = next_row;
            col = next_col;
        }

        // Remove all visited cells and set them to empty
        let step = self.set_all_visited_to_empty(&mut maze);
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
