use egui::InnerResponse;

use crate::app::{MazeAlgorithms, WindowState};
use crate::Main;

use super::Maze;

impl Main {
    pub fn generate_side_panel(&mut self, ctx: &egui::Context) -> InnerResponse<()> {
        return egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.set_min_width(250.0);

            ui.heading("Settings").highlight();

            ui.add_space(50.0);

            ui.heading("Map Settings");

            ui.add_space(15.0);

            ui.heading("algorithm");

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.settings.maze_algorithm,
                    MazeAlgorithms::Prims,
                    "Prim's algorithm",
                );
                ui.selectable_value(
                    &mut self.settings.maze_algorithm,
                    MazeAlgorithms::Kruskals,
                    "Kruskal's algorithm",
                );
            });
            ui.end_row();
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.settings.maze_algorithm,
                    MazeAlgorithms::DFS,
                    "DFS algorithm",
                );
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Maze Width: ");
                ui.add(
                    egui::DragValue::new(&mut self.settings.maze_size.0)
                        .range(5..=400)
                        .speed(0.1)
                        .max_decimals(0),
                );

                ui.add_space(5.0);

                ui.label("Maze Height: ");
                ui.add(
                    egui::DragValue::new(&mut self.settings.maze_size.1)
                        .range(5..=400)
                        .speed(0.1)
                        .max_decimals(0),
                );
            });

            ui.add_space(10.0);

            let create_window_btn = ui.button("Create window");

            if create_window_btn.clicked() {
                if self.windows.len() < 4 {
                    let mut maze = Maze::new(self.settings.maze_size.0, self.settings.maze_size.1);
            
                    match self.settings.maze_algorithm {
                        MazeAlgorithms::DFS => maze.init_dfs(),
                        MazeAlgorithms::Prims => maze.init_prims(),
                        MazeAlgorithms::Kruskals => maze.init_kruskals(),
                        // Add other algorithms as needed
                    }
            
                    let window = WindowState::new(
                        self.next_window_id,
                        format!("Maze {}", self.next_window_id),
                        maze,
                    );
            
                    self.windows.push(window);
                    self.next_window_id += 1;
                }
            }
            
            

            ui.add_space(5.0);

            let start_maze_generation_bth = ui.button("generate the mazes");

            if start_maze_generation_bth.clicked() {
                if self.windows.len() > 0 {
                    for window_index in 0..self.windows.len() {
                        self.windows[window_index].generating = true
                    }

                };
            }
        });
    }
}
