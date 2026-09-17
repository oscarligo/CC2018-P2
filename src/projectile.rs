use raylib::prelude::*;

use crate::maze::Maze;

const SPEED: f32 = 50.0;

pub struct Projectile {
    pub position: Vector2,
    pub marker: char,
    direction: Vector2,
}

impl Projectile {
    pub fn new(position: Vector2, angle: f32, block_size: usize) -> Self {
        let direction = Vector2::new(angle.cos(), angle.sin());
        Self {
            position: Vector2::new(
                position.x + direction.x * block_size as f32 / 2.0,
                position.y + direction.y * block_size as f32 / 2.0,
            ),
            marker: 'a',
            direction,
        }
    }

    pub fn update(&mut self, delta_time: f32, maze: &Maze, block_size: usize) -> bool {
        let next = Vector2::new(
            self.position.x + self.direction.x * SPEED * delta_time,
            self.position.y + self.direction.y * SPEED * delta_time,
        );
        let cell = maze
            .get(next.y as usize / block_size)
            .and_then(|row| row.get(next.x as usize / block_size));

        if matches!(cell, Some(' ' | 'p' | 'e' | 'w')) {
            self.position = next;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projectile_moves_until_it_hits_a_wall() {
        let maze = vec![
            vec!['+', '+', '+'],
            vec!['+', ' ', '+'],
            vec!['+', '+', '+'],
        ];
        let mut projectile = Projectile::new(Vector2::new(10.0, 15.0), 0.0, 10);

        assert!(projectile.update(0.05, &maze, 10));
        assert!(!projectile.update(0.05, &maze, 10));
    }
}
