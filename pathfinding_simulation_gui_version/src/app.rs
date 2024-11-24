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

                if self.windows.len() < 4 {

                    // Create a new WindowState
                    let window = WindowState {
                        id: self.next_window_id,
                    title: format!("Maze Window {}", self.next_window_id),
                    is_open: true,
                    // Initialize other fields as needed
                };
            
                self.windows.push(window);
                self.next_window_id += 1;
                
            }else {
                println!("cannot generate more windows")
            }
                // Optionally, generate the maze and associate it with the window
            }
            

            
        });
        
            egui::CentralPanel::default().show(ctx, |ui| {
                // Get the available size of the CentralPanel
                let available_size = ui.available_size();
        
                // Collect open windows
                let open_windows: Vec<&mut WindowState> = self.windows.iter_mut().filter(|w| w.is_open).collect();
        
                let num_windows = open_windows.len();
                if num_windows > 0 {


                    
                    let num_of_cells = open_windows.len() as f32; // N
                    let max_cols = 2; 
                    
                    let initial_cols = num_of_cells.sqrt().ceil() as usize;
                    


                    let num_of_cols = std::cmp::min(initial_cols, max_cols);
                    
                    let num_of_rows = (num_of_cells / num_of_cols as f32).ceil() as usize;

                    let mut window_iter = open_windows.into_iter();


                    let cell_width = available_size.x / num_of_cols as f32;
                    let cell_height = available_size.y / num_of_rows as f32;



                    egui::Grid::new("window_grid")
                                .num_columns(num_of_cols)
                                .show(ui, |ui| {
                                    for _row in 0..num_of_rows {
                                        for _col in 0..num_of_cols {
                                            if let Some(window) = window_iter.next() {
                                                ui.group(|ui| {

                                                    ui.set_min_size(egui::Vec2 { x: cell_width, y: cell_height });

                                                    // Display window content here
                                                    ui.heading(&window.title);
                                                    if ui.button("Close").clicked() {
                                                        window.is_open = false;
                                                    }
                                                    ui.separator();
                                                    // Replace with your maze rendering code
                                                    ui.label(format!("Content of {}", window.title));
                                                });
                                            } else {
                                                // No more windows, fill the cell with empty space
                                                ui.allocate_space(ui.available_size());
                                            }
                                            
                                        }
                                        ui.end_row();
                                    }
                                });
                    
                }
            });


    // Remove closed windows
    self.windows.retain(|window| window.is_open);

    }
}
