
use std::fmt;

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub enum MazeAlgorithms {
    Prims,
    Kruskals,
    DFS
}

impl fmt::Display for MazeAlgorithms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MazeAlgorithms::Prims => "Prim's Algorithm",
            MazeAlgorithms::Kruskals => "Kruskal's Algorithm",
            MazeAlgorithms::DFS => "DFS Algorithm"
        };
        write!(f, "{}", name)
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub enum PathfindingAlgorithms {
    Astar,
    Dijkstra,
    BFS,
    DFS,
}
