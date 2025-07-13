#![allow(dead_code)]
#![allow(unused_variables)]

// Imports
use std::io;
use rand::Rng;

const GRID_SIZE: usize = 50;
const GRID_NEIGHBROURS_TO_LIVE: u8 = 4;
// struct [[&str; GRID_SIZE]; GRID_SIZE]
const GRID_TILE_BLANK: &str = "..";
const GRID_TILE_WALL: &str = "##";

fn main() {

    println!("# Linear cave finder and indexer.");
    // User inputs
    println!("## Please enter config (choices) [default]... ");
    
    let mut user_probability_wall: String = String::new();
    let mut user_erosion_iterations: String = String::new();

    
    println!("- Wall fill (]0; 1[) [0.5]: ");
    io::stdin()
        .read_line(&mut user_probability_wall)
        .expect("(X) - Error reading the line.");
    
    let user_probability_wall: f64 = match user_probability_wall.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => 0.5f64,
    };


    println!("- Erosion iterations (int > 0) [5]");
    io::stdin()
        .read_line(&mut user_erosion_iterations)
        .expect("(X) - Error reading the line.");

    let user_erosion_iterations: usize = match user_erosion_iterations.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => 5usize,
    };

        
    // Generate grid
    println!("# Input received; generating grid");
    let grid_main: [[bool; GRID_SIZE]; GRID_SIZE] = generator(0.5, 5, GRID_TILE_BLANK, GRID_TILE_WALL);
    // print_grid(&grid_main);

    // Prevent auto closing
    println!("Enter to close...");
    let mut user_enter_to_quit: String = String::new();
    io::stdin()
        .read_line(&mut user_enter_to_quit)
        .expect("(X) - Error while quitting.");

}

fn print_grid(grid: &[[bool; GRID_SIZE]; GRID_SIZE]) {
    for line in grid.iter() {
		for tile in line.iter() {
			if *tile {
                print!("{}", GRID_TILE_WALL);
            } else {
                print!("{}", GRID_TILE_BLANK);
            }
            
		}
    println!();
    }
}

fn generator<'a>(probability_wall: f64, erosion_iteration: u32, character_air: &'a str, character_wall: &'a str) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    /*
     * Generate a basic random 2D world of caves with random erosion.
     */
    let mut rng = rand::rng();

    // Init the grid
    println!("## Initialized grid with: air='{character_air}'; wall='{character_wall}'.");
    let mut grid: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];
    
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
        for y in 0usize..grid.len() {
            let line = grid[y];
            for x in 0usize..line.len() {
                if grid[y][x] && neighbours([x, y], grid) < GRID_NEIGHBROURS_TO_LIVE {
                    grid[y][x] = false;
                } else if !grid[y][x] && neighbours([x, y], grid) > GRID_NEIGHBROURS_TO_LIVE {
                    grid[y][x] = true;
                }
            }
        }
        
        println!("- Erosion, iteration {iteration}: ");
        print_grid(&grid);
    }


    // Return finished grid
    grid
}

fn neighbours(tile_coord: [usize; 2usize], grid: [[bool; GRID_SIZE]; GRID_SIZE]) -> u8 {
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