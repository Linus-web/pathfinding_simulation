use crate::app::Maze;
use egui::{Color32, TextureHandle, TextureOptions};

pub struct WindowState {
    pub id: usize,        // Unique identifier for the window
    pub title: String,    // Title of the window
    pub is_open: bool,    // Whether the window is open
    pub generating: bool, // Whether the maze is currently being generated
    pub maze: Maze,       // The maze data

    pub maze_texture: Option<TextureHandle>, // Cached texture of the maze
    pub needs_redraw: bool,                  // Flag indicating if the maze needs to be redrawn
}

impl WindowState {
    pub fn new(id: usize, title: String, maze: Maze) -> Self {
        Self {
            id,
            title,
            is_open: true,
            generating: false,
            maze,
            maze_texture: None,
            needs_redraw: true,
        }
    }

    pub fn generate_maze_texture(&mut self, ctx: &egui::Context, size: [usize; 2]) {
        let width = size[0];
        let height = size[1];

        let mut image = egui::ColorImage::new([width, height], Color32::WHITE);

        // Render the maze onto the image
        self.render_maze_to_image(&mut image);

        let texture_options = TextureOptions::LINEAR;

        self.maze_texture =
            Some(ctx.load_texture(format!("maze_texture_{}", self.id), image, texture_options));

        self.needs_redraw = false;
    }

    fn render_maze_to_image(&self, image: &mut egui::ColorImage) {
        let maze_width = self.maze.width;
        let maze_height = self.maze.height;

        let pixels_per_cell_x = image.size[0] as f32 / maze_width as f32;
        let pixels_per_cell_y = image.size[1] as f32 / maze_height as f32;

        let stroke_color = Color32::BLACK;

        for y in 0..maze_height {
            for x in 0..maze_width {
                let node = &self.maze.grid[y][x];

                let x_pos = (x as f32 * pixels_per_cell_x) as usize;
                let y_pos = (y as f32 * pixels_per_cell_y) as usize;
                let x_end = ((x + 1) as f32 * pixels_per_cell_x) as usize;
                let y_end = ((y + 1) as f32 * pixels_per_cell_y) as usize;

                // Draw walls onto the image buffer
                // Top wall
                if node.walls[0] {
                    for xi in x_pos..x_end {
                        if y_pos < image.size[1] && xi < image.size[0] {
                            image.pixels[y_pos * image.size[0] + xi] = stroke_color;
                        }
                    }
                }
                // Left wall
                if node.walls[3] {
                    for yi in y_pos..y_end {
                        if yi < image.size[1] && x_pos < image.size[0] {
                            image.pixels[yi * image.size[0] + x_pos] = stroke_color;
                        }
                    }
                }
                // Right wall (only on last column)
                if x == maze_width - 1 && node.walls[1] {
                    for yi in y_pos..y_end {
                        if yi < image.size[1] && x_end - 1 < image.size[0] {
                            image.pixels[yi * image.size[0] + (x_end - 1)] = stroke_color;
                        }
                    }
                }
                // Bottom wall (only on last row)
                if y == maze_height - 1 && node.walls[2] {
                    for xi in x_pos..x_end {
                        if y_end - 1 < image.size[1] && xi < image.size[0] {
                            image.pixels[(y_end - 1) * image.size[0] + xi] = stroke_color;
                        }
                    }
                }
            }
        }
    }
}
