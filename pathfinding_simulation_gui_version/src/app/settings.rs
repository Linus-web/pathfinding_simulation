use crate::app::algorithms::{MazeAlgorithms, PathfindingAlgorithms};

pub struct AppSettings {
    pub maze_algorithm: MazeAlgorithms,
    pub pathfinding_algorithm: PathfindingAlgorithms,
    pub maze_size: (usize, usize),
    pub visualization_speed: i32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            maze_algorithm: MazeAlgorithms::Prims,
            pathfinding_algorithm: PathfindingAlgorithms::Astar,
            maze_size: (160, 100),
            visualization_speed: 100,
        }
    }
}
