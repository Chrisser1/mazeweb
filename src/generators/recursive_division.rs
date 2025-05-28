use js_sys::Math::random;

use crate::{cell::Cell, maze::{Maze, MazeChange}};
use super::generator::MazeGenerator;

pub struct RecursiveDivision;

impl MazeGenerator for RecursiveDivision {
    fn generate_maze_steps(&mut self, original_maze: &Maze) -> Option<Vec<Vec<MazeChange>>> {
        let mut steps = Vec::new();
        let mut maze = original_maze.clone();

        // Start with a blank maze
        let clearing = self.make_all_into(&mut maze, Cell::Empty);
        if !clearing.is_empty() {
            steps.push(clearing);
        }

        let mut stack = vec![Division {
            x: 0,
            y: 0,
            width: maze.width(),
            height: maze.height(),
            orientation: self.choose_orientation(maze.width(), maze.height()),
        }];

        while let Some(Division { x, y, width, height, orientation }) = stack.pop() {
            if width < 3 || height < 3 {
                continue;
            }

            let horizontal = matches!(orientation, Orientation::Horizontal);

            // Determine wall position (odd coordinate)
            let wall_x = if horizontal {
                0 // unused
            } else {
                x + 1 + 2 * ((random() * ((width.saturating_sub(2) / 2) as f64)).floor() as u32)
            };

            let wall_y = if horizontal {
                y + 1 + 2 * ((random() * ((height.saturating_sub(2) / 2) as f64)).floor() as u32)
            } else {
                0 // unused
            };

            // Determine passage position (even coordinate)
            let passage_x = if horizontal {
                x + 2 * ((random() * ((width / 2) as f64)).floor() as u32)
            } else {
                wall_x
            };

            let passage_y = if horizontal {
                wall_y
            } else {
                y + 2 * ((random() * ((height / 2) as f64)).floor() as u32)
            };

            let dx = if horizontal { 1 } else { 0 };
            let dy = if horizontal { 0 } else { 1 };

            let (wx, wy) = if horizontal { (x, wall_y) } else { (wall_x, y) };
            let length = if horizontal { width } else { height };

            let mut wall_step = Vec::new();

            for i in 0..length {
                let cx = wx + i * dx;
                let cy = wy + i * dy;

                if cx >= maze.width() || cy >= maze.height() {
                    continue;
                }

                let old = maze.get_cell(cy, cx);
                maze.set_cell(cy, cx, Cell::Wall);
                wall_step.push(MazeChange {
                    row: cy,
                    col: cx,
                    old,
                    new: Cell::Wall,
                });
            }

            if !wall_step.is_empty() {
                steps.push(wall_step);
            }

            // Remove the passage
            let mut passage_step = Vec::new();
            let old_passage = maze.get_cell(passage_y, passage_x);
            maze.set_cell(passage_y, passage_x, Cell::Changeing);
            passage_step.push(MazeChange {
                row: passage_y,
                col: passage_x,
                old: old_passage,
                new: Cell::Changeing,
            });
            steps.push(passage_step);

            let mut passage_step_2 = Vec::new();
            maze.set_cell(passage_y + dy, passage_x + dx, Cell::Empty);
            passage_step_2.push(MazeChange {
                row: passage_y,
                col: passage_x,
                old: Cell::Changeing,
                new: Cell::Empty,
            });
            steps.push(passage_step_2);

            // Subdivide
            let (nx1, ny1, w1, h1, nx2, ny2, w2, h2) = if horizontal {
                (
                    x, y, width, wall_y - y,
                    x, wall_y + 1, width, y + height - wall_y - 1
                )
            } else {
                (
                    x, y, wall_x - x, height,
                    wall_x + 1, y, x + width - wall_x - 1, height
                )
            };

            for (sx, sy, sw, sh) in [(nx1, ny1, w1, h1), (nx2, ny2, w2, h2)] {
                if sw >= 3 && sh >= 3 {
                    stack.push(Division {
                        x: sx,
                        y: sy,
                        width: sw,
                        height: sh,
                        orientation: self.choose_orientation(sw, sh),
                    });
                }
            }
        }

        Some(steps)
    }
}

#[derive(Clone, Copy)]
struct Division {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    orientation: Orientation,
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl RecursiveDivision {
    pub fn new() -> Self {
        RecursiveDivision
    }

    fn choose_orientation(&self, width: u32, height: u32) -> Orientation {
        if width < height {
            Orientation::Horizontal
        } else if height < width {
            Orientation::Vertical
        } else {
            if random() < 0.5 {
                Orientation::Vertical
            } else {
                Orientation::Horizontal
            }
        }
    }
}
