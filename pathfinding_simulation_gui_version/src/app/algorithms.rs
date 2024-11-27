use std::fmt;

#[derive(PartialEq)]
pub enum MazeAlgorithms {
    Prims,
    Kruskals,
    Dfs,
}

impl fmt::Display for MazeAlgorithms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MazeAlgorithms::Prims => "Prim's Algorithm",
            MazeAlgorithms::Kruskals => "Kruskal's Algorithm",
            MazeAlgorithms::Dfs => "DFS Algorithm",
        };
        write!(f, "{}", name)
    }
}

#[derive(PartialEq)]
pub enum PathfindingAlgorithms {
    Astar,
    Dijkstra,
    Bfs,
    Dfs,
}
