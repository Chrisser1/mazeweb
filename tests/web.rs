//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

extern crate mazeweb;
use mazeweb::Maze;

#[cfg(test)]
pub fn input_maze() -> Maze {
    let mut maze = Maze::new();
    maze.set_width(5);
    maze.set_height(5);
    maze.set_cells(&[(0,0), (0,2), (1,0), (1,2), (1,3), (3,0), (3,1), (3,2), (3,4), (4,1)], Cell::Wall); // Set the cells to walls
    maze.set_cell(&[(0,1)], Cell::Start);
    maze.set_cell(&[(0,3)], Cell::End);
    maze
}

#[cfg(test)]
pub fn expected_solution() -> Maze {
    let mut maze = Maze::new();
    maze.set_width(5);
    maze.set_height(5);
    maze.set_cells(&[(0,0), (0,2), (1,0), (1,2), (1,3), (3,0), (3,1), (3,2), (3,4), (4,1)], Cell::Wall);
    maze.set_cells(&[(0,1)], Cell::Start);
    maze.set_cells(&[(0,3)], Cell::End);
    maze.set_cells(&[(1,1), (2,1), (2,2), (2,3), (2,4), (1,4), (0,4)], Cell::Path);
    maze
}

#[wasm_bindgen_test]
pub fn test_maze_solve() {
    // Create maze with a start and end
    let mut input_maze = input_maze();

    // Create what the solved state of the maze should look like
    let mut solved_maze = expected_solution();

    // Solve the maze here

    assert_eq!(&input_maze.get_cells(), &expected_solution.get_cells());
}
