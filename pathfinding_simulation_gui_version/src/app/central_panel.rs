use egui::epaint::RectShape;
use egui::{Color32, InnerResponse, Pos2, Rect, Rounding, Stroke};

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
                                                
                                                
                                                let canvas_size = ui.available_size();
                                                let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::hover());

                                                let rect = response.rect;


                                                let rounding = Rounding::ZERO;
                                                let fill_color = Color32::from_rgb(255, 255, 255);
                                                let stroke = Stroke::new(1.0, Color32::from_rgb(0,0,0));

                                                painter.rect(rect, rounding, fill_color, stroke);
                                                
                                                
                                                
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

/*

 // Helper method to draw the maze
 fn draw_maze(maze: &Maze, painter: &egui::Painter, rect: Rect) {
    let num_cells_x = maze.width;
    let num_cells_y = maze.height;

    let cell_width = rect.width() / num_cells_x as f32;
    let cell_height = rect.height() / num_cells_y as f32;

    for y in 0..num_cells_y {
        for x in 0..num_cells_x {
            let node = &maze.grid[y][x];

            let x_pos = rect.min.x + x as f32 * cell_width;
            let y_pos = rect.min.y + y as f32 * cell_height;

            let cell_rect = Rect::from_min_size(
                Pos2::new(x_pos, y_pos),
                egui::vec2(cell_width, cell_height),
            );

            // Draw the cell background (optional)
            painter.rect_filled(cell_rect, Rounding::ZERO, Color32::WHITE);

            let stroke = Stroke::new(1.0, Color32::BLACK);

            // Draw the walls based on the `walls` array
            // walls[0]: Top, walls[1]: Right, walls[2]: Bottom, walls[3]: Left

            // Top wall
            if node.walls[0] {
                painter.line_segment(
                    [
                        Pos2::new(x_pos, y_pos),
                        Pos2::new(x_pos + cell_width, y_pos),
                    ],
                    stroke,
                );
            }
            // Right wall
            if node.walls[1] {
                painter.line_segment(
                    [
                        Pos2::new(x_pos + cell_width, y_pos),
                        Pos2::new(x_pos + cell_width, y_pos + cell_height),
                    ],
                    stroke,
                );
            }
            // Bottom wall
            if node.walls[2] {
                painter.line_segment(
                    [
                        Pos2::new(x_pos + cell_width, y_pos + cell_height),
                        Pos2::new(x_pos, y_pos + cell_height),
                    ],
                    stroke,
                );
            }
            // Left wall
            if node.walls[3] {
                painter.line_segment(
                    [
                        Pos2::new(x_pos, y_pos + cell_height),
                        Pos2::new(x_pos, y_pos),
                    ],
                    stroke,
                );
            }
        }
    }
}

 */

