// # LINEAR CAVE INDEXER and EROSION GENERATOR.

// Imports
use std::io;
use rand::Rng;

/// Define grid size.
const GRID_SIZE: usize = 50;
/// Define minimum number of neighbours required to stay alive (inclusive).
const GRID_NEIGHBROURS_TO_LIVE: u8 = 5;
/// Define minimum number of neighbours required to spontaneously come to life (exclusive).
const GRID_NEIGHBROURS_TO_BORN: u8 = 5;
/// Settings - Default fill percentage.
const DEFAULT_FILL: f64 = 0.7;
/// Settings - Default number of erosion iteration.
const DEFAULT_EROSION_ITERATIONS: u32 = 6;
/// Type grid.
type Grid = [[bool; GRID_SIZE]; GRID_SIZE];
/// UI - Structure of the possible state to print.
pub struct UiTileState {
    pub blank: &'static str,
    pub wall: &'static str,
}
/// UI - What to print for each state.
pub const UI_TILES_STATES: UiTileState = UiTileState {
    blank: "░░",
    wall: "██",
};

fn print_grid(grid: &[[bool; GRID_SIZE]; GRID_SIZE]) {
    for line in grid.iter() {
		for tile in line.iter() {
			if *tile {
                print!("{}", UI_TILES_STATES.wall);
            } else {
                print!("{}", UI_TILES_STATES.blank);
            }
            
		}
    println!();
    }
}

/// Generate a basic random 2D world of caves with random erosion.
fn generator(probability_wall: f64, erosion_iteration: u32, ui_tiles_states: UiTileState) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut rng = rand::rng();

    // Init the grid
    println!("## Initialized grid with: air='{}'; wall='{}'.", ui_tiles_states.blank, ui_tiles_states.wall);
    let mut grid: Grid = [[false; GRID_SIZE]; GRID_SIZE];
    
    // Generate randomly walls
    for line in grid.iter_mut() {
        for tile in line.iter_mut() {
            if rng.random_bool(probability_wall) {
                *tile = true;
            }
        }
    }

    println!("- First random generation: ");
    print_grid(&grid);

    // Erosion process

    for iteration in 1..=erosion_iteration {
        let mut grid_updated: Grid = [[false; GRID_SIZE]; GRID_SIZE];
        for y in 0usize..grid.len() {
            let line = grid[y];
            for x in 0usize..line.len() {
                if grid[y][x] && neighbours([x, y], grid) < GRID_NEIGHBROURS_TO_LIVE {
                    grid_updated[y][x] = false;
                } else if !grid[y][x] && neighbours([x, y], grid) > GRID_NEIGHBROURS_TO_BORN {
                    grid_updated[y][x] = true;
                } else {
                    grid_updated[y][x] = grid[y][x];
                }
            }
        }
        grid = grid_updated;
        
        println!("- Erosion, iteration {iteration}: ");
        print_grid(&grid);
    }


    // Return finished grid
    grid
}

fn neighbours(tile_coord: [usize; 2usize], grid: Grid) -> u8 {
    /*
     * Count neighbours wall around a tile. 
     */
    let relatives: [[i32; 2]; 8usize] = [
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1]
    ];
    let mut neighbours_count: u8 = 0;

    for relative in relatives {
        let absolute: [i32; 2usize] = [tile_coord[0] as i32 + relative[0], tile_coord[1] as i32 + relative[1]];
        // Check if tile is in grid
        if absolute[0] < 0 
            || 
            absolute[0] >= GRID_SIZE as i32 
            || 
            absolute[1] < 0 
            ||
            absolute[1] >= GRID_SIZE as i32 {

        } else if grid[absolute[0] as usize][absolute[1] as usize] {
            neighbours_count += 1;
        }


    }

    neighbours_count
}


/// # Run program.
fn main() {

    println!("# Linear cave finder and indexer.");
    // User inputs
    println!("## Please enter config (choices) [default]... ");
    
    let mut user_probability_wall: String = String::new();
    let mut user_erosion_iterations: String = String::new();

    
    println!("- Wall fill (]0; 1[) [{}]: ", DEFAULT_FILL);
    io::stdin()
        .read_line(&mut user_probability_wall)
        .expect("(X) - Error reading the line.");
    
    let user_probability_wall: f64 = match user_probability_wall.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => DEFAULT_FILL,
    };


    println!("- Erosion iterations (int > 0) [{}]", DEFAULT_EROSION_ITERATIONS);
    io::stdin()
        .read_line(&mut user_erosion_iterations)
        .expect("(X) - Error reading the line.");

    let user_erosion_iterations: u32 = match user_erosion_iterations.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => DEFAULT_EROSION_ITERATIONS,
    };

        
    // Generate grid
    println!("# Input received; generating grid");
    let grid_main: Grid = generator(user_probability_wall, user_erosion_iterations, UI_TILES_STATES);
    // print_grid(&grid_main);

    // Prevent auto closing
    println!("Enter to close...");
    let mut user_enter_to_quit: String = String::new();
    io::stdin()
        .read_line(&mut user_enter_to_quit)
        .expect("(X) - Error while quitting.");

}
