use raylib::prelude::*;

use crate::maze::Maze;

pub struct Portal {
    pub position: Vector2,
    pub marker: char,
    frame: usize,
    elapsed: f32,
}

impl Portal {
    pub fn from_maze(maze: &Maze, block_size: usize) -> Vec<Self> {
        maze.iter()
            .enumerate()
            .flat_map(|(row, cells)| {
                cells.iter().enumerate().filter_map(move |(column, &cell)| {
                    (cell == 'w').then(|| Self {
                        position: Vector2::new(
                            (column * block_size) as f32 + block_size as f32 / 2.0,
                            (row * block_size) as f32 + block_size as f32 / 2.0,
                        ),
                        marker: cell,
                        frame: 0,
                        elapsed: 0.0,
                    })
                })
            })
            .collect()
    }

    pub fn update(&mut self, delta_time: f32) {
        const FRAME_TIME: f32 = 0.1;
        self.elapsed += delta_time;
        let changes = (self.elapsed / FRAME_TIME) as usize;
        self.frame = (self.frame + changes) % 5;
        self.elapsed %= FRAME_TIME;
    }

    pub fn frame(&self) -> usize {
        self.frame
    }
}
