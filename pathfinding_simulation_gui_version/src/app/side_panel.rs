use egui::InnerResponse;

use crate::app::{MazeAlgorithms, WindowState};
use crate::Main;

use super::{Maze, PathfindingAlgorithms};

impl Main {
    pub fn generate_side_panel(&mut self, ctx: &egui::Context, integration_info : &eframe::IntegrationInfo) -> InnerResponse<()> {
        return egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.set_min_width(250.0);

            ui.heading("Settings").highlight();

            ui.add_space(50.0);

            self.generate_maze_side_section(ui);


            self.generate_pathfinding_side_section(ui);


            self.generate_settings_side_section(ui);


            self.generate_info_side_section(ui, integration_info);

        });
    }

    fn generate_maze_side_section(&mut self, ui: &mut egui::Ui) {

            ui.heading("Map generation");

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
                    MazeAlgorithms::Dfs,
                    "DFS algorithm",
                );
                ui.selectable_value(
                    &mut self.settings.maze_algorithm,
                    MazeAlgorithms::AldousBroder,
                    "Aldous-broder's algorithm",
                );
            });

            ui.separator();

            ui.add_space(10.0);

            let create_window_btn = ui.button("Create window");

            if create_window_btn.clicked() && self.windows.len() < 4 {
                let mut maze = Maze::new(self.settings.maze_size.0, self.settings.maze_size.1);

                match self.settings.maze_algorithm {
                    MazeAlgorithms::Dfs => maze.init_dfs(),
                    MazeAlgorithms::Prims => maze.init_prims(),
                    MazeAlgorithms::Kruskals => maze.init_kruskals(),
                    MazeAlgorithms::AldousBroder => maze.init_aldous_broder(),
                    // Add other algorithms as needed
                }



                let algorithm_name = self.settings.maze_algorithm.to_string();

                let window = WindowState::new(
                    self.next_window_id,
                    format!("Maze {} | {}", self.next_window_id, algorithm_name),
                    maze,
                );

                self.windows.push(window);
                self.next_window_id += 1;
            }

            ui.add_space(5.0);

            let start_maze_generation_bth = ui.button("generate the mazes");

            if start_maze_generation_bth.clicked() && !self.windows.is_empty() {
                for window_index in 0..self.windows.len() {
                    self.windows[window_index].generating = true
                }
            }

            ui.add_space(15.0);

            ui.separator();
    }


    fn generate_pathfinding_side_section(&mut self, ui: &mut egui::Ui) {


            ui.heading("Pathfinding");

            ui.add_space(15.0);

            ui.heading("algorithm");

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.settings.pathfinding_algorithm,
                    PathfindingAlgorithms::Astar,
                    "Prim's algorithm",
                );
                ui.selectable_value(
                    &mut self.settings.pathfinding_algorithm,
                    PathfindingAlgorithms::Dijkstra,
                    "Dijkstra's algorithm",
                );
            });
            ui.end_row();
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.settings.pathfinding_algorithm,
                    PathfindingAlgorithms::Dfs,
                    "DFS algorithm",
                );
                ui.selectable_value(
                    &mut self.settings.pathfinding_algorithm,
                    PathfindingAlgorithms::Bfs,
                    "BFS algorithm",
                );
            });

            ui.add_space(15.0);

            ui.separator();
            


    }


    fn generate_settings_side_section(&mut self, ui: &mut egui::Ui) {

            ui.heading("Settings");

            ui.add_space(15.0);

            ui.horizontal(|ui| {
                ui.label("visualization speed:");
                ui.add(
                    egui::DragValue::new(&mut self.settings.visualization_speed)
                        .range(1..=4000)
                        .speed(0.1)
                        .max_decimals(0),
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

            ui.add_space(15.0);

            ui.separator();

    }


    fn generate_info_side_section(&mut self, ui: &mut egui::Ui, integration_info: &eframe::IntegrationInfo) {
        ui.heading("Info");

        if let Some(cpu_usage) = integration_info.cpu_usage {


            let fps = (2.0 / cpu_usage + self.last_frame_fps as f32).floor();

            self.last_frame_fps = cpu_usage as usize;
            ui.label(format!("FPS: {:.2}", fps));
        } else {
            ui.label("FPS: N/A");
        }
    
        ui.add_space(15.0);
        ui.separator();
    }
}
