Pathfinding Simulation

Pathfinding Simulation is a Rust-based application for visualizing and generating mazes while testing various pathfinding algorithms such as A*, Dijkstra's, DFS, and BFS. The project also allows users to generate mazes using algorithms like Random DFS Maze Generation and Prim's Maze Generation.
Features

    Pathfinding Algorithms:
        A* Algorithm
        Dijkstra's Algorithm
        Depth-First Search (DFS)
        Breadth-First Search (BFS)

    Maze Generation:
        Random DFS Maze
        Prim's Maze

    Visualizations:
        Save mazes as PNG images.
        Animated progress bars for algorithm execution.

Directory Structure

pathfinding_simulation/
├── .github/
│   └── workflows/          # GitHub workflows for CI/CD
├── pathfinding_simulation/
│   ├── src/                # Source code for the project
│   │   ├── main.rs         # Main application entry point
│   │   ├── maze.rs         # Maze generation logic
│   │   └── save.rs         # Logic for saving maze images
│   ├── Cargo.toml          # Rust dependencies and project configuration
├── README.md               # Project documentation

Installation

    Clone the repository:

git clone https://github.com/your-username/pathfinding_simulation.git
cd pathfinding_simulation

Build the project:

cargo build --release

Run the application:

    cargo run

Usage

    Navigate through the menu to select:
        Pathfinding algorithms to test.
        Maze generation methods to visualize.

    Generated mazes will be saved as PNG images in the project directory.

    Customize the maze size and complexity in the source code (e.g., main.rs) if needed.

Running Tests

To ensure the project is working as expected, run the tests:

cargo test

GitHub Actions Workflow

This project includes a CI/CD pipeline via GitHub Actions:

    Automatically builds and tests the code on every push and pull request to the main branch.
    Configuration is in .github/workflows/rust.yml.

Contributing

Contributions are welcome! If you'd like to contribute:

    Fork the repository.
    Create a feature branch.
    Submit a pull request.

License

This project is licensed under the MIT License.
