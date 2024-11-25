use crate::app::Maze;
#[derive(serde::Deserialize, serde::Serialize)]

pub struct WindowState {
    pub id: usize,          // Unique identifier for the window
    pub title: String,      // Title of the window
    pub is_open: bool,      // Whether the window is open
    pub generating: bool,
    pub grid : Maze,
}
