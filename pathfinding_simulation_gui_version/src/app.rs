// app.rs

mod algorithms;
mod central_panel;
mod maze;
mod settings;
mod side_panel;
mod window_state;

pub use algorithms::{MazeAlgorithms, PathfindingAlgorithms};
pub use maze::Maze;
pub use settings::AppSettings;
pub use window_state::WindowState;


pub struct Main {
    settings: AppSettings,
    windows: Vec<WindowState>,
    next_window_id: usize,
}

impl Default for Main {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
            windows: Vec::new(),
            next_window_id: 0,
        }
    }
}

impl Main {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Main {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Flag to indicate if any maze is still generating
        let mut any_maze_generating = false;

        // Perform maze generation steps
        for window in &mut self.windows {
            if window.generating {
                // Perform a number of steps based on visualization speed
                let generation_continues = window.maze.step(self.settings.visualization_speed as usize);
                if !generation_continues {
                    // Maze generation is complete
                    window.generating = false;
                } else {
                    any_maze_generating = true; // At least one maze is still generating
                }
                window.needs_redraw = true; // Maze has changed, needs to be redrawn
            }
        }

        // Proceed with your normal UI code
        self.generate_side_panel(ctx);
        self.generate_central_panel(ctx);

        // Request a repaint if any window needs to be redrawn
        if any_maze_generating || self.windows.iter().any(|w| w.needs_redraw) {
            ctx.request_repaint();
        }
    }
}
