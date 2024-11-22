use console::{Term, Key};
use std::io::{self};

mod maze;
mod save;

use maze::Maze;
use save::save_maze_image;

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
mod tests {
    use super::*;

    #[test]
    fn test_menu_navigation() {
        let start_menu_options = vec!["Test algorithms", "Generate a random maze", "Settings", "Exit"];
        let mut selected_index = 0;

        selected_index += 1;
        assert_eq!(start_menu_options[selected_index], "Generate a random maze");

        selected_index -= 1;
        assert_eq!(start_menu_options[selected_index], "Test algorithms");
    }
}
