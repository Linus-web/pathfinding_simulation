
use egui::InnerResponse;


use crate::Main;
use crate::app::{MazeAlgorithms,WindowState};

use super::Maze;


impl Main{

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
                    ui.selectable_value(&mut self.settings.maze_algorithm, MazeAlgorithms::Prims, "Prim's algorithm");
                    ui.selectable_value(&mut self.settings.maze_algorithm, MazeAlgorithms::Kruskals, "Kruskal's algorithm");
                });
                ui.end_row();
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.settings.maze_algorithm, MazeAlgorithms::DFS, "DFS algorithm");
                });
            
            ui.add_space(10.0);
    
            ui.horizontal(|ui| {
    
                // Input field for Maze Height
            ui.label("Maze Height: ");
            ui.add(
                egui::DragValue::new(&mut self.settings.maze_size.0)
                    .range(5..=400)
                    .speed(0.1)
                    .max_decimals(0),
            );
    
            ui.add_space(5.0);
    
            // Input field for Maze Width
            ui.label("Maze Width: ");
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

                    let window = WindowState {
                        id: self.next_window_id,
                    title: format!("Window {}", self.settings.maze_algorithm),
                    is_open: true,
                    generating: false,
                    grid: Maze::new(self.settings.maze_size.0, self.settings.maze_size.1)
                };
            
                self.windows.push(window);
                self.next_window_id += 1;
                
            }     
            }

            let start_maze_generation_bth = ui.button("generate the mazes");

            if start_maze_generation_bth.clicked(){

                if self.windows.len() > 0 {

                    for window_index in 0..self.windows.len(){

                        self.windows[window_index].generating = true

                    }
                    
                };
                
            }     
            
    
            
        });
    


}


}

