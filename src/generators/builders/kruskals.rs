use js_sys::Math::random;

use crate::{generators::generator::MazeGenerator, maze::{Maze, MazeChange}, utils};


// A simple Union-Find data structure for Kruskal's algorithm with path compression and union by size.
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        // find with path-compression
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // compress
        while x != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            return false; // already connected
        }
        // union by size
        if self.size[root_a] < self.size[root_b] {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        } else {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        }
        true
    }
}



pub struct Kruskals;

impl MazeGenerator for Kruskals {
    fn generate_maze_steps(&mut self, original_maze: &Maze) -> Option<Vec<Vec<MazeChange>>> {
        let mut steps = Vec::new();
        let mut maze = original_maze.clone();
        let h = maze.height() as usize;
        let w = maze.width() as usize;
        let total_cells = h * w;

        // Make all into walls
        let walling_step: Vec<MazeChange> = self.add_all_walls(&mut maze);
        if !walling_step.is_empty() {
            steps.push(walling_step);
        }

        // Collect all walls
        let mut walls = Vec::new();
        for r in 0..maze.height() {
            for c in 0..maze.width() {
                if r + 1 < maze.height() {
                    walls.push((r, c, 1, 0));
                }
                if c + 1 < maze.width() {
                    walls.push((r, c, 0, 1));
                }
            }
        }

        // Shuffle the walls to randomize the order
        for i in (1..walls.len()).rev() {
            // pick a random index j in [0..=i]
            let j = (random() * ((i + 1) as f64)).floor() as usize;
            walls.swap(i, j);
        }

        let mut uf = UnionFind::new(total_cells);
        let mut carved = 0;
        for &(r, c, dr, dc) in &walls {
            if carved == total_cells - 1 {
                break;
            }
            let nr = (r as i32 + dr as i32) as u32;
            let nc = (c as i32 + dc as i32) as u32;

            let idx1 = (r as usize) * w + (c as usize);
            let idx2 = (nr as usize) * w + (nc as usize);

            // only carve if these cells weren’t already connected
            if uf.union(idx1, idx2) {
                let mut step = Vec::new();
                self.remove_wall_between(&mut maze, r, c, nr, nc, &mut step);
                steps.push(step);
                carved += 1;
            }
        }

        Some(steps)
    }
}

impl Kruskals {
    pub fn new() -> Self {
        Kruskals
    }
}
