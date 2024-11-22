use console::{Term, Key};
use std::io::{self};

mod maze;
mod save;

use save::save_maze_image;

#[derive(PartialEq,Debug)]
enum MenuState {
    MainMenu,
    AlgorithmMenu,
    MazeMenu,
    SettingsMenu
}


const ASCII_ART : &str  = "██████╗  █████╗ ████████╗██╗  ██╗███████╗██╗███╗   ██╗██████╗ ███████╗██████╗ 
██╔══██╗██╔══██╗╚══██╔══╝██║  ██║██╔════╝██║████╗  ██║██╔══██╗██╔════╝██╔══██╗
██████╔╝███████║   ██║   ███████║█████╗  ██║██╔██╗ ██║██║  ██║█████╗  ██████╔╝
██╔═══╝ ██╔══██║   ██║   ██╔══██║██╔══╝  ██║██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗
██║     ██║  ██║   ██║   ██║  ██║██║     ██║██║ ╚████║██████╔╝███████╗██║  ██║
╚═╝     ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝     ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝";





fn main() -> io::Result<()> {


   

    let term = Term::stdout();
    let mut selected_index = 0;
    let mut menu_stack: Vec<MenuState> = vec![MenuState::MainMenu];

    loop {
        term.clear_screen()?;

        match menu_stack.last() {
            Some(MenuState::MainMenu) => {
                let options = ["Test algorithms", "Generate a random maze", "Settings", "Exit"];
                print_menu(&options, selected_index);
                if handle_key_input(&term, &mut selected_index, options.len(), &mut menu_stack)? {
                    break;
                }
            }
            Some(MenuState::AlgorithmMenu) => {
                let options = ["Back", "A*", "Dijkstra's", "DFS", "BFS"];
                print_menu(&options, selected_index);
                if handle_key_input(&term, &mut selected_index, options.len(), &mut menu_stack)? {
                    break;
                }
            }
            Some(MenuState::MazeMenu) => {
                let options = ["Back", "Random DFS Maze", "Prims Maze"];
                print_menu(&options, selected_index);
                if handle_key_input(&term, &mut selected_index, options.len(), &mut menu_stack)? {
                    break;
                }
            }
            Some(MenuState::SettingsMenu) => {
                let options = ["Back"];
                print_menu(&options, selected_index);
                if handle_key_input(&term, &mut selected_index, options.len(), &mut menu_stack)? {
                    break;
                }
            }
            _ => break,
        }
    
    }

    Ok(())
        
}





fn print_menu(menu: &[&str], selected_index: usize) {
    println!("{}", ASCII_ART);
    println!("\n==== Welcome to My Game ====\n");
    for (index, option) in menu.iter().enumerate() {
        if index == selected_index {
            println!("> {}", option);
        } else {
            println!("  {}", option);
        }
    }
}


fn handle_key_input(
    term: &console::Term,
    selected_index: &mut usize,
    menu_len: usize,
    menu_stack: &mut Vec<MenuState>,
) -> io::Result<bool> {
    match term.read_key()? {
        Key::ArrowUp => {
            if *selected_index > 0 {
                *selected_index -= 1;
            }
        }
        Key::ArrowDown => {
            if *selected_index < menu_len - 1 {
                *selected_index += 1;
            }
        }
        Key::Enter => {
            match menu_stack.last() {
                Some(MenuState::MainMenu) => match *selected_index {
                    0 => menu_stack.push(MenuState::AlgorithmMenu), 
                    1 => menu_stack.push(MenuState::MazeMenu),      
                    2 => menu_stack.push(MenuState::SettingsMenu),  
                    3 => return Ok(true),
                    _ => {}
                },
                Some(MenuState::AlgorithmMenu) => match *selected_index {
                    0 => {
                        menu_stack.pop(); 
                    }
                    1 => println!("Running A* algorithm..."),
                    2 => println!("Running Dijkstra's algorithm..."),
                    3 => println!("Running DFS algorithm..."),
                    4 => println!("Running BFS algorithm..."),
                    _ => {}
                },
                Some(MenuState::MazeMenu) => match *selected_index {
                    0 => {
                        menu_stack.pop(); 
                    }
                    1 => {
                        let mut maze = Maze::new(20, 20);
                        maze.dfs_maze();
                        if let Err(e) = save_maze_image(&maze, "maze.png") {
                            eprintln!("Failed to save maze: {}", e);
                        } else {
                            println!("Maze saved successfully as maze.png");
                        }
                        println!("Press Enter to continue...");
                        let mut input = String::new();
                        let _ = std::io::stdin().read_line(&mut input);
                    }
                    2 => {
                        let mut maze = Maze::new(20,20);
                        maze.prims_maze();
                        if let Err(e) = save_maze_image(&maze, "maze.png") {
                            eprintln!("Failed to save maze: {}", e);
                        } else {
                            println!("Maze saved successfully as maze.png");
                        }
                        println!("Press Enter to continue...");
                        let mut input = String::new();
                        let _ = std::io::stdin().read_line(&mut input);
                    }
                    _ => {}
                },
                Some(MenuState::SettingsMenu) => {
                    println!("Settings functionality not yet implemented.");
                    menu_stack.pop(); 
                }
                None => return Ok(true), 
            }
            *selected_index = 0;
        }
        _ => {}
    }
    Ok(false)
}




#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn test_main_menu_options() {
        let menu_options = vec!["Test algorithms", "Generate a random maze", "Settings", "Exit"];
        assert_eq!(menu_options.len(), 4);
        assert_eq!(menu_options[0], "Test algorithms");
        assert_eq!(menu_options[3], "Exit");
    }

    #[test]
    fn test_algorithm_menu_options() {
        let menu_options = vec!["Back", "A*", "Dijkstra's", "DFS", "BFS"];
        assert_eq!(menu_options.len(), 5);
        assert_eq!(menu_options[1], "A*");
        assert_eq!(menu_options[4], "BFS");
    }

    #[test]
    fn test_menu_stack_navigation() {
        let mut menu_stack: Vec<MenuState> = vec![MenuState::MainMenu];

        menu_stack.push(MenuState::AlgorithmMenu);
        assert_eq!(menu_stack.last(), Some(&MenuState::AlgorithmMenu));

        menu_stack.push(MenuState::MazeMenu);
        assert_eq!(menu_stack.last(), Some(&MenuState::MazeMenu));

        menu_stack.pop();
        assert_eq!(menu_stack.last(), Some(&MenuState::AlgorithmMenu));

        menu_stack.pop();
        assert_eq!(menu_stack.last(), Some(&MenuState::MainMenu));
    }

    #[test]
    fn test_handle_key_input_navigation() {
        let mut selected_index = 0;
        let menu_len = 4;
        let mut menu_stack: Vec<MenuState> = vec![MenuState::MainMenu];
    
        // Simulate ArrowDown key press
        assert!(!handle_simulated_key_input(Key::ArrowDown, &mut selected_index, menu_len, &mut menu_stack).unwrap());
        assert_eq!(selected_index, 1);
    
        // Simulate ArrowUp key press
        assert!(!handle_simulated_key_input(Key::ArrowUp, &mut selected_index, menu_len, &mut menu_stack).unwrap());
        assert_eq!(selected_index, 0);
    
        // Simulate Enter key press on the first option (AlgorithmMenu)
        assert!(!handle_simulated_key_input(Key::Enter, &mut selected_index, menu_len, &mut menu_stack).unwrap());
        assert_eq!(menu_stack.last(), Some(&MenuState::AlgorithmMenu));
    
        // Simulate Back action to return to MainMenu
        selected_index = 0;
        assert!(!handle_simulated_key_input(Key::Enter, &mut selected_index, menu_len, &mut menu_stack).unwrap());
        assert_eq!(menu_stack.last(), Some(&MenuState::MainMenu));
    }
    

    #[test]
    fn test_print_menu() {
        let menu_options = vec!["Option 1", "Option 2", "Option 3"];
        let selected_index = 1;

        // Capture the printed output
        let output = std::panic::catch_unwind(|| {
            print_menu(&menu_options, selected_index);
        });

        assert!(output.is_ok());
    }

    #[test]
    fn test_ascii_art_display() {
        // Check if ASCII_ART constant is non-empty and properly loaded
        assert!(!ASCII_ART.is_empty());
        assert!(ASCII_ART.contains("██████╗"));
    }

    #[test]
    fn test_maze_creation_and_saving() {
        let mut maze = Maze::new(10, 10);
        maze.dfs_maze();
        let result = save_maze_image(&maze, "test_maze.png");
        assert!(result.is_ok(), "Failed to save the maze image");
    }

    #[test]
    fn test_menu_state_transitions() {
        let mut menu_stack: Vec<MenuState> = vec![MenuState::MainMenu];

        // Transition to AlgorithmMenu
        menu_stack.push(MenuState::AlgorithmMenu);
        assert_eq!(menu_stack.last(), Some(&MenuState::AlgorithmMenu));

        // Transition to MazeMenu
        menu_stack.push(MenuState::MazeMenu);
        assert_eq!(menu_stack.last(), Some(&MenuState::MazeMenu));

        // Go back to AlgorithmMenu
        menu_stack.pop();
        assert_eq!(menu_stack.last(), Some(&MenuState::AlgorithmMenu));

        // Go back to MainMenu
        menu_stack.pop();
        assert_eq!(menu_stack.last(), Some(&MenuState::MainMenu));
    }
}


#[cfg(test)]
fn handle_simulated_key_input(
    key: Key,
    selected_index: &mut usize,
    menu_len: usize,
    menu_stack: &mut Vec<MenuState>,
) -> io::Result<bool> {
    match key {
        Key::ArrowUp => {
            if *selected_index > 0 {
                *selected_index -= 1;
            }
        }
        Key::ArrowDown => {
            if *selected_index < menu_len - 1 {
                *selected_index += 1;
            }
        }
        Key::Enter => {
            match menu_stack.last() {
                Some(MenuState::MainMenu) => match *selected_index {
                    0 => menu_stack.push(MenuState::AlgorithmMenu),
                    1 => menu_stack.push(MenuState::MazeMenu),
                    2 => menu_stack.push(MenuState::SettingsMenu),
                    3 => return Ok(true), // Exit
                    _ => {}
                },
                Some(MenuState::AlgorithmMenu) => {
                    if *selected_index == 0 {
                        menu_stack.pop(); // Back to MainMenu
                    }
                }
                Some(MenuState::MazeMenu) => {
                    if *selected_index == 0 {
                        menu_stack.pop(); // Back to MainMenu
                    }
                }
                Some(MenuState::SettingsMenu) => {
                    menu_stack.pop(); // Back to MainMenu
                }
                None => return Ok(true), // Exit
            }
            *selected_index = 0; // Reset selected index on state transition
        }
        _ => {}
    }
    Ok(false)
}
