
#[cfg(not(test))]
use indicatif::{ProgressBar, ProgressStyle};
use rand::thread_rng;
use rand::seq::{IteratorRandom,SliceRandom};
mod node;

pub use node::Node;


pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Node>>,
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

        Maze { width, height, grid }
    }

    pub fn dfs_maze(&mut self) {

        let mut stack: Vec<(usize,usize)>  = Vec::new();
        let mut rng = thread_rng();


        #[cfg(not(test))]
        let bar = ProgressBar::new(self.height as u64 *self.width as u64);


        #[cfg(not(test))]
        bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));
        #[cfg(not(test))]
        bar.set_message("Generating maze...");
        let start_x = rand::random::<usize>() % self.width as usize;
        let start_y = rand::random::<usize>() % self.height as usize;

        self.grid[start_y][start_x].visited = true;

        stack.push((start_y,start_x));

        
        while let Some((x,y)) = stack.pop() {
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

            if let Some(&(nx, ny, current_wall, neighbor_wall)) = neighbors.choose(&mut rng) {
                self.grid[y][x].walls[current_wall] = false;
                self.grid[ny][nx].walls[neighbor_wall] = false;

                self.grid[ny][nx].visited = true;
                stack.push((x, y));
                stack.push((nx, ny));



                #[cfg(not(test))]
                bar.inc(1);
            }

        }


        #[cfg(not(test))]
        bar.finish();

    }


    pub fn prims_maze(&mut self) {
        let mut walls: Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut rng = thread_rng();
    
        #[cfg(not(test))]
        let bar = ProgressBar::new((self.height * self.width) as u64);
    
        #[cfg(not(test))]
        bar.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        #[cfg(not(test))]
        bar.set_message("Generating maze...");
    
        let start_x = rand::random::<usize>() % self.width;
        let start_y = rand::random::<usize>() % self.height;
    
        self.grid[start_y][start_x].visited = true;
    
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
    
        
        walls.shuffle(&mut rng);


        while !walls.is_empty() {
            if let Some(index) = (0..walls.len()).choose(&mut rng) {
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
                    #[cfg(not(test))]
                    bar.inc(1);
                }
            }
        }
        #[cfg(not(test))]
        bar.finish();
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_creation() {
        let maze = Maze::new(10, 10);

        assert_eq!(maze.width, 10);
        assert_eq!(maze.height, 10);

        for row in maze.grid.iter() {
            for node in row.iter() {
                assert!(!node.visited);
                assert_eq!(node.walls, [true, true, true, true]);
            }
        }
    }

    #[test]
    fn test_dfs_maze() {
        let mut maze = Maze::new(10, 10);
        maze.dfs_maze();

        for row in maze.grid.iter() {
            for node in row.iter() {
                assert!(node.visited);
            }
        }
    }

    #[test]
    fn test_dfs_maze_edge_cases() {
        let mut small_maze = Maze::new(1, 1);
        small_maze.dfs_maze();

        assert!(small_maze.grid[0][0].visited);

        let mut large_maze = Maze::new(1000, 1000);
        large_maze.dfs_maze();

        assert!(large_maze.grid[0][0].visited); 
    }


    #[test]
    fn test_prims_maze() {
        let mut maze = Maze::new(10, 10);
        maze.prims_maze();

        for row in maze.grid.iter() {
            for node in row.iter() {
                assert!(node.visited);
            }
        }

        let mut has_open_wall = false;
        for row in maze.grid.iter() {
            for node in row.iter() {
                if node.walls.contains(&false) {
                    has_open_wall = true;
                    break;
                }
            }
        }
        assert!(has_open_wall, "Maze should have at least one open wall between cells.");
    }

    #[test]
    fn test_prims_maze_edge_cases() {
        let mut small_maze = Maze::new(1, 1);
        small_maze.prims_maze();

        assert!(small_maze.grid[0][0].visited);
        assert_eq!(small_maze.grid[0][0].walls, [true, true, true, true]); 

        let mut large_maze = Maze::new(1000, 1000);
        large_maze.prims_maze();

        for row in large_maze.grid.iter() {
            for node in row.iter() {
                assert!(node.visited);
            }
        }
    }

    #[test]
    fn test_prims_maze_randomness() {
        let mut maze1 = Maze::new(10, 10);
        let mut maze2 = Maze::new(10, 10);

        maze1.prims_maze();
        maze2.prims_maze();

        let mut identical = true;
        for y in 0..10 {
            for x in 0..10 {
                if maze1.grid[y][x].walls != maze2.grid[y][x].walls {
                    identical = false;
                    break;
                }
            }
            if !identical {
                break;
            }
        }
        assert!(!identical, "Mazes generated by Prim's algorithm should be different.");
    }


}

