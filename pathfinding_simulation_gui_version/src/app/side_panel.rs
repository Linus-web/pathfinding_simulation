
use egui::InnerResponse;



use crate::Main;
use crate::app::{MazeAlgorithms,WindowState};



impl Main{

    pub fn generate_side_panel(&mut self, ctx: &egui::Context) -> InnerResponse<()> {

        return egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.set_min_width(250.0);
    
    
            ui.heading("Settings").highlight();
            
    
            ui.add_space(50.0);
    
    
            ui.heading("Map Settings");

            ui.add_space(10.0);

            ui.heading("algorithm");
    
            ui.add_space(20.0);

            for options in MazeAlgorithms {
                
            }
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.settings.maze_algorithm, MazeAlgorithms::Prims, "Prim's algorithm");
                    ui.selectable_value(&mut self.settings.maze_algorithm, MazeAlgorithms::Kruskals, "Kruskal's algorithm");
                });
                ui.end_row();
            
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
    
    
            let generate_maze_btn = ui.button("Generate Maze");
    
            if generate_maze_btn.clicked() {
    
                if self.windows.len() < 4 {
    
                    // Create a new WindowState
                    let window = WindowState {
                        id: self.next_window_id,
                    title: format!("Maze Window {}", self.next_window_id),
                    is_open: true,
                    // Initialize other fields as needed
                };
            
                self.windows.push(window);
                self.next_window_id += 1;
                
            }else {
                println!("cannot generate more windows")
            }
                // Optionally, generate the maze and associate it with the window
            }
            
    
            
        });
    


}


}

