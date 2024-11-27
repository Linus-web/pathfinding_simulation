use image::{RgbImage, Rgb};
#[cfg(not(test))]
use std::time::Duration;
use crate::maze::Maze; // Import the Maze struct from the maze module

#[cfg(not(test))]
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[cfg(test)]
use indicatif::ProgressBar;

pub fn save_maze_image(maze: &Maze, filename: &str) -> Result<(), std::io::Error> {
    let cell_size = 5;

    let wall_thickness = 1;

    #[cfg(not(test))]
    let multi_progress = MultiProgress::new();
    #[cfg(not(test))]
    let generate_bar = multi_progress.add(ProgressBar::new(maze.height as u64 * maze.width as u64));
    #[cfg(not(test))]
    generate_bar.set_message("Generating image...");
    #[cfg(not(test))]
    generate_bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    #[cfg(not(test))]
    let img = generate_maze_image_with_bar(maze, cell_size, wall_thickness, Some(&generate_bar));
    #[cfg(test)]
    let img = generate_maze_image_with_bar(maze, cell_size, wall_thickness, None);

    #[cfg(not(test))]
    generate_bar.finish_with_message("Image generation complete.");
    #[cfg(not(test))]
    let save_bar = multi_progress.add(ProgressBar::new_spinner());
    #[cfg(not(test))]
    save_bar.set_message("Saving image...");
    #[cfg(not(test))]
    save_bar.enable_steady_tick(Duration::from_millis(100));

    let result = img.save(filename).map_err(|e| {
        #[cfg(not(test))]
        save_bar.finish_with_message("Failed to save image.");
        std::io::Error::new(std::io::ErrorKind::Other, e)
    });

    if result.is_ok() {
        #[cfg(not(test))]
        save_bar.finish_with_message("Image saved successfully.");
    }

    result
}

pub fn generate_maze_image_with_bar(
    maze: &Maze,
    cell_size: u32,
    wall_thickness: u32,
    bar: Option<&ProgressBar>, // Use Option to include or exclude the ProgressBar
) -> RgbImage {
    let img_width = maze.width as u32 * cell_size;
    let img_height = maze.height as u32 * cell_size;
    let mut img = RgbImage::new(img_width, img_height);

    let wall_color = Rgb([0, 0, 0]); 
    let background_color = Rgb([255, 255, 255]); 

    // Fill the image with the background color
    for x in 0..img_width {
        for y in 0..img_height {
            img.put_pixel(x, y, background_color);
        }
    }

    // Draw walls based on maze structure
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

            // Increment the progress bar if available
            if let Some(bar) = bar {
                bar.inc(1);
            }
        }
    }

    img
}
