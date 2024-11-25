// app.rs

mod side_panel;
mod central_panel;
mod algorithms;
mod window_state;
mod settings;

pub use window_state::WindowState;
pub use algorithms::{MazeAlgorithms, PathfindingAlgorithms};
pub use settings::AppSettings;



#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Main {
    counter: i32,
    user_input: String,
    settings: AppSettings,
    windows: Vec<WindowState>,
    next_window_id: usize,
}



impl Default for Main {
    fn default() -> Self {
        Self {
            counter: 0,
            user_input: "".to_string(),
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
