use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub position: Vector2,
    pub angle: f32,
    pub fov: f32,
    pub move_speed: f32,
    pub rotation_speed: f32,
}

impl Player {
    pub fn new(position: Vector2, angle: f32, move_speed: f32, rotation_speed: f32, fov: f32) -> Self {
        Player { position, angle, move_speed, rotation_speed, fov }
    }

    fn try_move(&mut self, distance: f32, maze: &Maze, block_size: usize) {
        let next_x = self.position.x + distance * self.angle.cos();
        let next_y = self.position.y + distance * self.angle.sin();

        let cell = maze
            .get(next_y as usize / block_size)
            .and_then(|row| row.get(next_x as usize / block_size));

        if matches!(cell, Some(' ' | 'p' | 'g' | 'w')) {
            self.position = Vector2::new(next_x, next_y);
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn move_forward(&mut self, maze: &Maze, block_size: usize) {
        self.try_move(self.move_speed, maze, block_size);
    }

    pub fn move_backward(&mut self, maze: &Maze, block_size: usize) {
        self.try_move(-self.move_speed, maze, block_size);
    }

    pub fn rotate_left(&mut self) {
        self.rotate(-self.rotation_speed);
    }

    pub fn rotate_right(&mut self) {
        self.rotate(self.rotation_speed);
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle = (self.angle + radians).rem_euclid(2.0 * PI);
    }



    pub fn set_initial_position(&mut self, maze: &Maze, block_size: usize) {
        for (row_index, row) in maze.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                if cell == 'p' {
                    self.position = Vector2::new(
                        (col_index * block_size) as f32 + (block_size as f32 / 2.0),
                        (row_index * block_size) as f32 + (block_size as f32 / 2.0),
                    );
                    return;
                }
            }
        }
    }

    
}
