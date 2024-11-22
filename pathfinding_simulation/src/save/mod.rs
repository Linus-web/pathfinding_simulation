use image::{RgbImage, Rgb};
use std::time::Duration;
use crate::maze::Maze; // Import the Maze struct from the maze module

use indicatif::{MultiProgress,ProgressBar, ProgressStyle};


pub fn save_maze_image(maze: &Maze, filename: &str) -> Result<(), std::io::Error> {
    let cell_size = 5;
    let wall_thickness = 1;

    let multi_progress = MultiProgress::new();

    let generate_bar = multi_progress.add(ProgressBar::new(maze.height as u64 * maze.width as u64));
    generate_bar.set_message("Generating image...");
    generate_bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    let img = generate_maze_image_with_bar(maze, cell_size, wall_thickness, &generate_bar);
    generate_bar.finish_with_message("Image generation complete.");

    let save_bar = multi_progress.add(ProgressBar::new_spinner());
    save_bar.set_message("Saving image...");
    save_bar.enable_steady_tick(Duration::from_millis(100));

    let result = img.save(filename).map_err(|e| {
        save_bar.finish_with_message("Failed to save image.");
        std::io::Error::new(std::io::ErrorKind::Other, e)
    });

    if result.is_ok() {
        save_bar.finish_with_message("Image saved successfully.");
    }

    result
}

pub fn generate_maze_image_with_bar(
    maze: &Maze,
    cell_size: u32,
    _wall_thickness: u32,
    bar: &ProgressBar,
) -> RgbImage {
    let img_width = maze.width as u32 * cell_size;
    let img_height = maze.height as u32 * cell_size;
    let mut img = RgbImage::new(img_width, img_height);

    let wall_color = Rgb([0, 0, 0]); 
    let background_color = Rgb([255, 255, 255]); 

    
    for x in 0..img_width {
        for y in 0..img_height {
            img.put_pixel(x, y, background_color);
        }
    }

    
    for row in &maze.grid {
        for node in row {
            let x = node.x as u32 * cell_size;
            let y = node.y as u32 * cell_size;

            
            if node.walls[0] {
                for i in 0..cell_size {
                    img.put_pixel(x + i, y, wall_color);
                }
            }
            if node.walls[1] {
                for i in 0..cell_size {
                    img.put_pixel(x + cell_size - 1, y + i, wall_color);
                }
            }
            if node.walls[2] {
                for i in 0..cell_size {
                    img.put_pixel(x + i, y + cell_size - 1, wall_color);
                }
            }
            if node.walls[3] {
                for i in 0..cell_size {
                    img.put_pixel(x, y + i, wall_color);
                }
            }

            bar.inc(1);
        }
    }

    img
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_maze_image() {
        let maze = Maze::new(10, 10);

        // Generate a simple maze
        let mut maze = Maze::new(10, 10);
        maze.dfs_maze();

        // Save the maze image
        let result = save_maze_image(&maze, "test_maze.png");
        assert!(result.is_ok());

        // Check if the file was created
        assert!(fs::metadata("test_maze.png").is_ok());

        // Clean up the file after testing
        fs::remove_file("test_maze.png").unwrap();
    }
}
