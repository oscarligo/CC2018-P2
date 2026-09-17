use std::fs::File;
use std::io::{BufRead, BufReader};
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(file_path: &str) -> Maze {
    let file = File::open(file_path).expect("Failed to open maze file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    let x = xo ;
    let y = yo ;

    match cell {
        '+' => {
            // Draw wall
            framebuffer.set_current_color(Color::BLACK);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }
        ' ' => {
            // Draw empty space
            framebuffer.set_current_color(Color::GRAY);    
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }
        '|' => {
            // Handle vertical walls
            framebuffer.set_current_color(Color::BLACK);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }
        '-' => {
            // Handle horizontal walls  
            framebuffer.set_current_color(Color::BLACK);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }

        '#' => {
            // Handle special wall
            framebuffer.set_current_color(Color::BLACK);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }

        '&' => {
            // Handle another special wall
            framebuffer.set_current_color(Color::BLACK);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }

        'w' => {
            // Handle portal
            framebuffer.set_current_color(Color::PURPLE);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }

        'e' => {
            // Handle enemy
            framebuffer.set_current_color(Color::RED);
            for i in 0..block_size {
                for j in 0..block_size {
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }
    
        _ => {
            // Handle other characters 
            framebuffer.set_current_color(Color::GRAY);
            for i in 0..block_size {
                for j in 0..block_size {    
                    framebuffer.set_pixel((x + i) as u32, (y + j) as u32);
                }
            }
        }
    }


}

pub fn render_maze(
    framebuffer: &mut Framebuffer, 
    maze: &Maze,
    block_size: usize) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
}