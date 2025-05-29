use js_sys::Math::random;
use crate::{cell::{CellType, WALL_E, WALL_N, WALL_S, WALL_W}, maze::{Maze, MazeChange}};
use super::generator::MazeGenerator;

pub struct RecursiveDivision;

impl MazeGenerator for RecursiveDivision {
    fn generate_maze_steps(&mut self, original_maze: &Maze) -> Option<Vec<Vec<MazeChange>>> {
        let mut steps = Vec::new();
        let mut maze = original_maze.clone();

        // Start with all walls removed
        let clear_step = self.remove_all_walls(&mut maze);
        if !clear_step.is_empty() {
            steps.push(clear_step);
        }

        let mut stack = vec![Division {
            x: 0,
            y: 0,
            width: maze.width(),
            height: maze.height(),
            orientation: self.choose_orientation(maze.width(), maze.height()),
        }];

        while let Some(Division { x, y, width, height, orientation }) = stack.pop() {
            if width < 2 || height < 2 {
                continue;
            }

            let horizontal = matches!(orientation, Orientation::Horizontal);

            // Choose wall position within correct bounds
            let wx = if horizontal { x } else { x + 1 + (random() * ((width - 2) as f64)).floor() as u32 };
            let wy = if horizontal { y + 1 + (random() * ((height - 2) as f64)).floor() as u32 } else { y };

            // Choose passage position on the wall line
            let px = if horizontal { x + (random() * (width as f64)).floor() as u32 } else { wx };
            let py = if horizontal { wy } else { y + (random() * (height as f64)).floor() as u32 };

            let dx = if horizontal { 1 } else { 0 };
            let dy = if horizontal { 0 } else { 1 };
            let length = if horizontal { width } else { height };

            let mut wall_step = Vec::new();

            // Place wall
            for i in 0..length {
                let cx = wx + i * dx;
                let cy = wy + i * dy;

                if cx >= maze.width() || cy >= maze.height() {
                    continue;
                }

                if horizontal {
                    self.add_wall_dir(&mut maze, cy, cx, -1, 0, &mut wall_step);
                } else {
                    self.add_wall_dir(&mut maze, cy, cx, 0, -1, &mut wall_step);
                }
            }

            if !wall_step.is_empty() {
                steps.push(wall_step);
            }

            // Explicitly visualize removing walls for the passage
            let mut carve = Vec::new();
            if horizontal {
                self.remove_wall_dir(&mut maze, py, px, -1, 0, &mut carve);
            } else {
                self.remove_wall_dir(&mut maze, py, px, 0, -1, &mut carve);
            }
            if !carve.is_empty() {
                steps.push(carve);
            }

            // Subdivide the remaining areas
            let (nx1, ny1, w1, h1, nx2, ny2, w2, h2) = if horizontal {
                // top region
                (x, y, width, wy - y,
                 // bottom region
                 x, wy, width, y + height - wy)
            } else {
                // left region
                (x, y, wx - x, height,
                 // right region
                 wx, y, x + width - wx, height)
            };

            for (sx, sy, sw, sh) in [(nx1, ny1, w1, h1), (nx2, ny2, w2, h2)] {
                if sw >= 2 && sh >= 2 {
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
