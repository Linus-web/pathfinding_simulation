

#[derive(serde::Deserialize, serde::Serialize)]

pub struct WindowState {
    pub id: usize,          // Unique identifier for the window
    pub title: String,      // Title of the window
    pub is_open: bool,      // Whether the window is open
    // Add other fields as needed, e.g., content, position, size, etc.
}
