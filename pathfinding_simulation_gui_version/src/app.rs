// app.rs
use std::fmt;


#[derive(serde::Deserialize, serde::Serialize)]

struct WindowState {
    id: usize,          // Unique identifier for the window
    title: String,      // Title of the window
    is_open: bool,      // Whether the window is open
    // Add other fields as needed, e.g., content, position, size, etc.
}


#[derive(serde::Serialize, serde::Deserialize,PartialEq)]
enum MazeAlgorithm{ Prims, Kruskals}

impl fmt::Display for MazeAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MazeAlgorithm::Prims => "Prim's Algorithm",
            MazeAlgorithm::Kruskals => "Kruskal's Algorithm",
        };
        write!(f, "{}", name)
    }
}


#[derive(serde::Serialize, serde::Deserialize,PartialEq)]
enum PathfinginAlgorithms { Astar, Dijkstra, BFS, DFS}


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppSettings {
    maze_algorithm : MazeAlgorithm,
    pathfinding_algorithm : PathfinginAlgorithms,
    maze_size : (usize,usize),
    visualization_speed : i32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self{
            maze_algorithm : MazeAlgorithm::Prims,
            pathfinding_algorithm: PathfinginAlgorithms::Astar,
            maze_size: (100,100),
            visualization_speed: 1,
        }
    }
}


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
        


        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.set_min_width(250.0);


            ui.heading("Settings").highlight();
            

            ui.add_space(50.0);


            ui.heading("Maze Settings");

            ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.settings.maze_algorithm, MazeAlgorithm::Prims, "Prim's algorithm");
                    ui.selectable_value(&mut self.settings.maze_algorithm, MazeAlgorithm::Kruskals, "Kruskal's algorithm");
                });
                ui.end_row();
            
            ui.add_space(10.0);

            ui.horizontal(|ui| {

                // Input field for Maze Height
            ui.label("Maze Height: ");
            ui.add(
                egui::DragValue::new(&mut self.settings.maze_size.0)
                    .range(5..=400)
                    .speed(0.1)
                    .max_decimals(0),
            );

            ui.add_space(5.0);

            // Input field for Maze Width
            ui.label("Maze Width: ");
            ui.add(
                egui::DragValue::new(&mut self.settings.maze_size.1)
                    .range(5..=400)
                    .speed(0.1)
                    .max_decimals(0),
            );

            });

            ui.add_space(10.0);


            let generate_maze_btn = ui.button("Generate Maze");

            if generate_maze_btn.clicked() {
                // Create a new WindowState
                let window = WindowState {
                    id: self.next_window_id,
                    title: format!("Maze Window {}", self.next_window_id),
                    is_open: true,
                    // Initialize other fields as needed
                };
            
                self.windows.push(window);
                self.next_window_id += 1;
            
                // Optionally, generate the maze and associate it with the window
            }
            

            
        });
        
            egui::CentralPanel::default().show(ctx, |ui| {
                // Your central panel content
            
                // Use a temporary vector to collect indices of windows to remove
                let mut windows_to_remove = Vec::new();
            
                for (index, window) in self.windows.iter_mut().enumerate() {
                    if window.is_open {
                        egui::Window::new(&window.title)
                            .id(egui::Id::new(window.id))
                            .resizable(true)
                            .collapsible(true)
                            .open(&mut window.is_open) // Binds the window's open state
                            .show(ctx, |ui| {
                                // Replace with your maze rendering code
                                ui.label(format!("Content of {}", window.title));

                                
                            });
                    } else {
                        // Mark the window for removal
                        windows_to_remove.push(index);
                    }
                }
            
                // Remove closed windows from the collection
                for index in windows_to_remove.iter().rev() {
                    self.windows.remove(*index);
                }
            });


    }
}
