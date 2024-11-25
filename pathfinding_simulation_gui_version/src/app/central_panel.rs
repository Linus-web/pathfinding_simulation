use egui::InnerResponse;

use crate::Main;

use crate::app::WindowState;


impl Main {


    pub fn generate_central_panel(&mut self, ctx: &egui::Context) -> InnerResponse<()> {
        return egui::CentralPanel::default().show(ctx, |ui| {

            let available_size = ui.available_size();
    
            let open_windows: Vec<&mut WindowState> = self.windows.iter_mut().filter(|w| w.is_open).collect();
    
            let num_windows = open_windows.len();
            if num_windows > 0 {

                
                let num_of_cells = open_windows.len() as f32; // N
                let max_cols = 2; 
                
                let initial_cols = num_of_cells.sqrt().ceil() as usize;

                let num_of_cols = std::cmp::min(initial_cols, max_cols);
                
                let num_of_rows = (num_of_cells / num_of_cols as f32).ceil() as usize;

                let mut window_iter = open_windows.into_iter();

                let cell_width = available_size.x / num_of_cols as f32;
                let cell_height = available_size.y / num_of_rows as f32;



                egui::Grid::new("window_grid")
                            .num_columns(num_of_cols)
                            .show(ui, |ui| {
                                for _row in 0..num_of_rows {
                                    for _col in 0..num_of_cols {
                                        if let Some(window) = window_iter.next() {
                                            ui.group(|ui| {

                                                ui.set_min_size(egui::Vec2 { x: cell_width, y: cell_height });
                                                ui.set_max_width(cell_width);

                                                ui.vertical(|ui| {
                                                ui.heading(&window.title);
                                                if ui.button("Close").clicked() {
                                                    window.is_open = false;
                                                    self.next_window_id -= 1;
                                                }
                                                ui.separator();
                                                ui.label(format!("Content of {}", window.title));
                                                });
                                                
                                            });
                                        } else {
                                            ui.allocate_space(ui.available_size());
                                            ui.vertical_centered(|ui| {
                                                ui.horizontal_centered(|ui|{
                                                    ui.heading("Start by generating a map...");
                                                });
                                            });
                                        }
                                        
                                    }
                                    ui.end_row();
                                }
                            });
                
            }
            self.windows.retain(|window| window.is_open);

        });

    }

}