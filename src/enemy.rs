use raylib::prelude::*;

use crate::maze::Maze;

pub struct Enemy {
    pub position: Vector2,
    pub marker: char,
    frame: usize,
    elapsed: f32,
    frame_time: f32,
}

impl Enemy {
    pub fn new(position: Vector2, marker: char, frame_time: f32) -> Self {
        Self {
            position,
            marker,
            frame: 0,
            elapsed: 0.0,
            frame_time,
        }
    }

    pub fn from_maze(maze: &Maze, block_size: usize) -> Vec<Self> {
        maze.iter()
            .enumerate()
            .flat_map(|(row, cells)| {
                cells.iter().enumerate().filter_map(move |(column, &cell)| {
                    (cell == 'e').then(|| {
                        Self::new(
                            Vector2::new(
                                (column * block_size) as f32 + block_size as f32 / 2.0,
                                (row * block_size) as f32 + block_size as f32 / 2.0,
                            ),
                            cell,
                            0.35,
                        )
                    })
                })
            })
            .collect()
    }

    pub fn update(&mut self, delta_time: f32) {
        self.elapsed += delta_time;
        let changes = (self.elapsed / self.frame_time) as usize;
        self.frame = (self.frame + changes) % 2;
        self.elapsed %= self.frame_time;
    }

    pub fn frame(&self) -> usize {
        self.frame
    }
}
