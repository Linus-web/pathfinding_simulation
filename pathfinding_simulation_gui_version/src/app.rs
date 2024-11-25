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
        self.generate_side_panel(ctx);

        self.generate_central_panel(ctx);
    }
}
