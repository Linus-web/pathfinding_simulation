// app.rs

use std::time::Instant;

use rayon::prelude::*;


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


    last_frame_time: Option<Instant>,
    smoothed_fps: f64,

}

impl Default for Main {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
            windows: Vec::new(),
            next_window_id: 0,

            last_frame_time: None,
            smoothed_fps: 60.0, 

        }
    }
}

impl Main {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

/*
impl eframe::App for Main {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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

        let integration_info = &frame.info();

        self.generate_side_panel(ctx, integration_info);
        self.generate_central_panel(ctx);

        // Request a repaint if any window needs to be redrawn
        if any_maze_generating || self.windows.iter().any(|w| w.needs_redraw) {
            ctx.request_repaint();
        }
    }
}

*/

impl eframe::App for Main {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate the desired frame duration based on desired FPS
        let desired_fps = self.settings.desired_fps; // You can set this in your settings
        let frame_duration = std::time::Duration::from_secs_f64(1.0 / desired_fps as f64);

        let now = Instant::now();
        if let Some(last_time) = self.last_frame_time {
            let delta = now.duration_since(last_time);
            let fps = 1.0 / delta.as_secs_f64();

            // Apply Exponential Moving Average for smoothing
            let smoothing_factor = 0.05; // Adjust this between 0 and 1 (lower means more smoothing)
            self.smoothed_fps = (fps * smoothing_factor) + (self.smoothed_fps * (1.0 - smoothing_factor));
        }
        self.last_frame_time = Some(now);

        // Flag to indicate if any maze is still generating
        let any_maze_generating = std::sync::atomic::AtomicBool::new(false);

        // Perform maze generation steps in parallel
        self.windows
            .par_iter_mut()
            .for_each(|window| {
                if window.generating {
                    // Perform a number of steps based on visualization speed
                    let generation_continues = window.maze.step(
                        self.settings.visualization_speed as usize,
                        &mut window.generation_time,
                    );
                    if !generation_continues {
                        // Maze generation is complete
                        window.generating = false;
                    } else {
                        any_maze_generating.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    window.needs_redraw = true; // Maze has changed, needs to be redrawn
                }
            });

        // Proceed with your normal UI code
        self.generate_side_panel(ctx);
        self.generate_central_panel(ctx);

        // Request a repaint after the desired frame duration
        ctx.request_repaint_after(frame_duration);
    }
}
