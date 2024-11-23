// main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pathfinding_simulation_with_gui::Main;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "maze and pathfinding algorithm showcase",
        native_options,
        Box::new(|cc| Ok(Box::new(Main::new(cc)))),
    )
}
