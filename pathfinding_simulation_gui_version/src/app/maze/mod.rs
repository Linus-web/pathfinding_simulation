use rand::seq::{IteratorRandom, SliceRandom};
use rand::{thread_rng, Rng};
pub mod node;

pub use node::Node;

pub enum MazeGenerator {
    DFS {
        stack: Vec<(usize, usize)>,
    },
    Prims {
        walls: Vec<(usize, usize, usize, usize)>,
    },
    // You can add other algorithms here
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Node>>,
    pub generator: Option<MazeGenerator>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| Node {
                        x,
                        y,
                        visited: false,
                        walls: [true, true, true, true],
                        is_current: false,
                        in_stack: false,
                    })
                    .collect()
            })
            .collect();

        Maze {
            width,
            height,
            grid,
            generator: None,
        }
    }

    /// Initializes the maze for DFS algorithm.
    pub fn init_dfs(&mut self) {
        let mut rng = thread_rng();
        let start_x = rng.gen_range(0..self.width);
        let start_y = rng.gen_range(0..self.height);
        self.grid[start_y][start_x].visited = true;

        self.generator = Some(MazeGenerator::DFS {
            stack: vec![(start_x, start_y)],
        });
    }

    pub fn init_prims(&mut self) {
        let mut rng = thread_rng();
        let start_x = rng.gen_range(0..self.width);
        let start_y = rng.gen_range(0..self.height);
        self.grid[start_y][start_x].visited = true;

        let mut walls = Vec::new();
        // Add initial walls surrounding the starting cell
        if start_x > 0 {
            walls.push((start_x, start_y, start_x - 1, start_y)); // Left wall
        }
        if start_x < self.width - 1 {
            walls.push((start_x, start_y, start_x + 1, start_y)); // Right wall
        }
        if start_y > 0 {
            walls.push((start_x, start_y, start_x, start_y - 1)); // Top wall
        }
        if start_y < self.height - 1 {
            walls.push((start_x, start_y, start_x, start_y + 1)); // Bottom wall
        }

        self.generator = Some(MazeGenerator::Prims { walls });
    }

    /// Initializes the maze for Kruskal's algorithm (placeholder).
    pub fn init_kruskals(&mut self) {
        // Implement Kruskal's algorithm initialization if needed
        // For now, we'll set the generator to None
        self.generator = None;
    }

    pub fn step(&mut self, steps: usize) -> bool {
        for _ in 0..steps {
            // Take the generator out of self.generator
            let mut generator = match self.generator.take() {
                Some(gen) => gen,
                None => return false, // Generation already complete
            };

            // Process the generator
            let generation_complete = match &mut generator {
                MazeGenerator::DFS { stack } => {
                    if !self.dfs_step(stack) {
                        true // Generation complete
                    } else {
                        false
                    }
                }
                MazeGenerator::Prims { walls } => {
                    if !self.prims_step(walls) {
                        true // Generation complete
                    } else {
                        false
                    }
                }
            };

            if generation_complete {
                self.generator = None; // Generation complete
                return false;
            } else {
                // Put the generator back into self.generator
                self.generator = Some(generator);
            }
        }
        true // Generation still in progress
    }

    fn dfs_step(&mut self, stack: &mut Vec<(usize, usize)>) -> bool {
        // Clear the `is_current` and `in_stack` flags for all nodes
        for row in &mut self.grid {
            for node in row {
                node.is_current = false;
                node.in_stack = false;
            }
        }

        // Set `in_stack` for nodes in the stack
        for &(x, y) in stack.iter() {
            self.grid[y][x].in_stack = true;
        }

        if let Some((x, y)) = stack.pop() {
            // Same logic as before but for a single step
            let mut neighbors = Vec::new();

            if x > 0 && !self.grid[y][x - 1].visited {
                neighbors.push((x - 1, y, 3, 1));
            }
            if x < self.width - 1 && !self.grid[y][x + 1].visited {
                neighbors.push((x + 1, y, 1, 3));
            }
            if y > 0 && !self.grid[y - 1][x].visited {
                neighbors.push((x, y - 1, 0, 2));
            }
            if y < self.height - 1 && !self.grid[y + 1][x].visited {
                neighbors.push((x, y + 1, 2, 0));
            }

            if !neighbors.is_empty() {
                let mut rng = thread_rng();
                let &(nx, ny, current_wall, neighbor_wall) = neighbors.choose(&mut rng).unwrap();

                self.grid[y][x].walls[current_wall] = false;
                self.grid[ny][nx].walls[neighbor_wall] = false;
                self.grid[ny][nx].visited = true;

                stack.push((x, y));
                stack.push((nx, ny));
            }
            true
        } else {
            false // Stack is empty, generation complete
        }
    }

    fn prims_step(&mut self, walls: &mut Vec<(usize, usize, usize, usize)>) -> bool {
        // Clear the `is_current` and `in_stack` flags for all nodes
        for row in &mut self.grid {
            for node in row {
                node.is_current = false;
                node.in_stack = false;
            }
        }

        // Set `in_stack` for nodes adjacent to walls in the walls list
        for &(x1, y1, x2, y2) in walls.iter() {
            self.grid[y2][x2].in_stack = true;
        }

        if let Some(index) = (0..walls.len()).choose(&mut thread_rng()) {
            let (x1, y1, x2, y2) = walls.remove(index);

            if !self.grid[y2][x2].visited {
                self.grid[y2][x2].visited = true;

                if x1 == x2 {
                    if y1 > y2 {
                        self.grid[y1][x1].walls[0] = false; // Top wall
                        self.grid[y2][x2].walls[2] = false; // Bottom wall
                    } else {
                        self.grid[y1][x1].walls[2] = false; // Bottom wall
                        self.grid[y2][x2].walls[0] = false; // Top wall
                    }
                } else {
                    if x1 > x2 {
                        self.grid[y1][x1].walls[3] = false; // Left wall
                        self.grid[y2][x2].walls[1] = false; // Right wall
                    } else {
                        self.grid[y1][x1].walls[1] = false; // Right wall
                        self.grid[y2][x2].walls[3] = false; // Left wall
                    }
                }

                // Add neighboring walls
                if x2 > 0 && !self.grid[y2][x2 - 1].visited {
                    walls.push((x2, y2, x2 - 1, y2));
                }
                if x2 < self.width - 1 && !self.grid[y2][x2 + 1].visited {
                    walls.push((x2, y2, x2 + 1, y2));
                }
                if y2 > 0 && !self.grid[y2 - 1][x2].visited {
                    walls.push((x2, y2, x2, y2 - 1));
                }
                if y2 < self.height - 1 && !self.grid[y2 + 1][x2].visited {
                    walls.push((x2, y2, x2, y2 + 1));
                }
            }
            true
        } else {
            false // Walls list is empty, generation complete
        }
    }
}
