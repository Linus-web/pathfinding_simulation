use std::time::{Duration, Instant};

use rand::seq::{IteratorRandom, SliceRandom};
use rand::{thread_rng, Rng};
pub mod node;

pub use node::Node;


pub enum MazeGenerator {
    Dfs {
        stack: Vec<(usize, usize)>,
    },
    Prims {
        walls: Vec<(usize, usize, usize, usize)>,
    },
    AldousBroder {
        current: (usize,usize),
        unvisited: usize,
    },
    Kruskal {
        edges: Vec<(usize, usize, usize, usize)>, // All edges (walls)
        sets: Vec<usize>, // Union-Find data structure to track connected regions
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

    /// Initializes the maze for Dfs algorithm.
    pub fn init_dfs(&mut self) {
        let mut rng = thread_rng();
        let start_x = rng.gen_range(0..self.width);
        let start_y = rng.gen_range(0..self.height);
        self.grid[start_y][start_x].visited = true;

        self.generator = Some(MazeGenerator::Dfs {
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

    pub fn init_aldous_broder(&mut self){

        let mut rng = thread_rng();
        let start_x = rng.gen_range(0..self.width);
        let start_y = rng.gen_range(0..self.height);
        self.grid[start_y][start_x].visited = true;

        self.generator = Some(MazeGenerator::AldousBroder { 
            current: (start_x,start_y), 
            unvisited: self.width*self.height -1 
        });

    }

    pub fn init_kruskals(&mut self) {
        let mut edges = Vec::new();
        let mut sets = vec![0; self.width * self.height];
    
        // Initialize Union-Find sets
        for i in 0..sets.len() {
            sets[i] = i;
        }
    
        // Collect all possible edges (walls) between cells
        for y in 0..self.height {
            for x in 0..self.width {
                if x < self.width - 1 {
                    // Right wall
                    edges.push((x, y, x + 1, y));
                }
                if y < self.height - 1 {
                    // Bottom wall
                    edges.push((x, y, x, y + 1));
                }
            }
        }
    
        // Shuffle edges for randomness
        edges.shuffle(&mut thread_rng());
        
        // Initialize the generator
        self.generator = Some(MazeGenerator::Kruskal { edges, sets });
    }
  
    pub fn step(&mut self, steps: usize, generation_time: &mut Duration) -> bool {
        let start = Instant::now(); // Start timing all steps
    
        for _ in 0..steps {
            // Take the generator out of self.generator
            let mut generator = match self.generator.take() {
                Some(gen) => gen,
                None => return false, // Generation already complete
            };
    
            // Process the generator
            let generation_complete = match &mut generator {
                MazeGenerator::Dfs { stack } => !self.dfs_step(stack),
                MazeGenerator::Prims { walls } => !self.prims_step(walls),
                MazeGenerator::AldousBroder { current, unvisited } => !self.aldous_broder_step(current, unvisited),
                MazeGenerator::Kruskal { edges, sets } => !self.kruskal_step(edges,sets),
            };
    
            if generation_complete {
                self.generator = None; // Generation complete
                *generation_time += start.elapsed(); // Accumulate total time
                return false;
            }
    
            // Restore the generator for the next iteration
            self.generator = Some(generator);
        }
    
        *generation_time += start.elapsed(); // Accumulate total time for all steps
        true // Generation still in progress
    }
    
    

    fn dfs_step(&mut self, stack: &mut Vec<(usize, usize)>) -> bool {

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

    fn aldous_broder_step(&mut self,current: &mut (usize, usize),unvisited: &mut usize,) -> bool {
        let mut rng = thread_rng();
        let (x, y) = *current;
    
        // Randomly pick a valid neighbor
        let neighbors: Vec<(usize, usize, usize, usize)> = vec![
            (x.wrapping_sub(1), y, 3, 1), // Left
            (x + 1, y, 1, 3),            // Right
            (x, y.wrapping_sub(1), 0, 2), // Up
            (x, y + 1, 2, 0),            // Down
        ]
        .into_iter()
        .filter(|&(nx, ny, _, _)| nx < self.width && ny < self.height) // Keep within bounds
        .collect();
    
        // If no neighbors, return early
        if neighbors.is_empty() {
            return false;
        }
    
        // Choose a random neighbor
        if let Some(&(nx, ny, current_wall, neighbor_wall)) = neighbors.choose(&mut rng) {
            if !self.grid[ny][nx].visited {
                // Carve passage
                self.grid[y][x].walls[current_wall] = false;
                self.grid[ny][nx].walls[neighbor_wall] = false;
                self.grid[ny][nx].visited = true;
    
                // Decrement unvisited count
                *unvisited -= 1;
            }
            // Move to the chosen neighbor
            *current = (nx, ny);
        }
    
        // Return whether there are still unvisited cells
        *unvisited > 0
    }
    
    fn kruskal_step(&mut self, edges: &mut Vec<(usize, usize, usize, usize)>, sets: &mut Vec<usize>) -> bool {
        while let Some((x1, y1, x2, y2)) = edges.pop() {
            let index1 = y1 * self.width + x1; // Flattened index
            let index2 = y2 * self.width + x2;

            self.grid[y1][x1].visited = true;
            self.grid[y2][x2].visited = true;
    
            if self.find(sets, index1) != self.find(sets, index2) {
                // Merge the sets
                self.union(sets, index1, index2);
    
                // Remove the wall between the cells
                if x1 == x2 {
                    // Vertical wall
                    if y1 < y2 {
                        self.grid[y1][x1].walls[2] = false; // Bottom
                        self.grid[y2][x2].walls[0] = false; // Top
                    } else {
                        self.grid[y1][x1].walls[0] = false; // Top
                        self.grid[y2][x2].walls[2] = false; // Bottom
                    }
                } else {
                    // Horizontal wall
                    if x1 < x2 {
                        self.grid[y1][x1].walls[1] = false; // Right
                        self.grid[y2][x2].walls[3] = false; // Left
                    } else {
                        self.grid[y1][x1].walls[3] = false; // Left
                        self.grid[y2][x2].walls[1] = false; // Right
                    }
                }
                return true; // Continue generating
            }
        }
        false // No edges left, generation complete
    }
    
}






impl Maze {
    fn find(&mut self, sets: &mut [usize], node: usize) -> usize {
        if sets[node] != node {
            sets[node] = self.find(sets, sets[node]); // Path compression
        }
        sets[node]
    }

    fn union(&mut self, sets: &mut [usize], a: usize, b: usize) {
        let root_a = self.find(sets, a);
        let root_b = self.find(sets, b);
        if root_a != root_b {
            sets[root_b] = root_a; // Merge the two sets
        }
    }
}