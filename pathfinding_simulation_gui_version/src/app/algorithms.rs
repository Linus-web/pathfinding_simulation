use std::fmt;

#[derive(PartialEq)]
pub enum MazeAlgorithms {
    Prims,
    Kruskals,
    Dfs,
    AldousBroder,
}

impl fmt::Display for MazeAlgorithms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MazeAlgorithms::Prims => "Prim's Algorithm",
            MazeAlgorithms::Kruskals => "Kruskal's Algorithm",
            MazeAlgorithms::Dfs => "DFS Algorithm",
            MazeAlgorithms::AldousBroder => "Aldous-B Algorithm",
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

impl fmt::Display for PathfindingAlgorithms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PathfindingAlgorithms::Astar => "Astar Algorithm",
            PathfindingAlgorithms::Bfs => "BFS Algorithm",
            PathfindingAlgorithms::Dfs => "DFS Algorithm",
            PathfindingAlgorithms::Dijkstra => "Dijkstra's Algorithm",
        };
        write!(f, "{}", name)
    }
}
