use glam::Vec3A;
use crate::cube::Cube;
use crate::material::Material;

pub struct WorldGenerator {
    pub size_x: usize,
    pub size_y: usize,
    pub size_z: usize,
    pub block_size: f32,
    pub offset_x: i32,
    pub offset_z: i32,
}

impl WorldGenerator {
    pub fn new(size_x: usize, size_y: usize, size_z: usize, block_size: f32) -> Self {
        Self {
            size_x,
            size_y,
            size_z,
            block_size,
            offset_x: 0,
            offset_z: 0,
        }
    }

    pub fn shift(&mut self, dx: i32, dz: i32) {
        self.offset_x += dx;
        self.offset_z += dz;
    }

    pub fn generate(&self, material: Material) -> Vec<Cube> {
        let total = self.size_x * self.size_y * self.size_z;
        let mut grid = vec![0u8; total];

        let idx = |x: usize, y: usize, z: usize| -> usize {
            (y * self.size_z + z) * self.size_x + x
        };

        for x in 0..self.size_x {
            let world_x = (x as i32 + self.offset_x) as f32;

            for z in 0..self.size_z {
                let world_z = (z as i32 + self.offset_z) as f32;

                let plateau_wave = ((world_x * 0.12).sin() + (world_z * 0.12).cos()) * 0.5;
                let plateau_extra = if plateau_wave > 0.25 { 1 } else { 0 };

                let px = ((world_x * 0.45).sin() * 0.5 + 0.5).max(0.0);
                let pz = ((world_z * 0.45).cos() * 0.5 + 0.5).max(0.0);
                let peak_factor = (px * pz).powi(6);

                let peak_extra = if peak_factor > 0.70 {
                    5
                } else if peak_factor > 0.45 {
                    3
                } else if peak_factor > 0.20 {
                    1
                } else {
                    0
                };

                let extra_height = plateau_extra + peak_extra;
                let max_y = extra_height.min(self.size_y.saturating_sub(1));

                for y in 0..=max_y {
                    grid[idx(x, y, z)] = 1;
                }
            }
        }

        let mut cubes = Vec::new();
        let sx = self.size_x;
        let sy = self.size_y;
        let sz = self.size_z;

        for y in 0..sy {
            for z in 0..sz {
                for x in 0..sx {
                    if grid[idx(x, y, z)] == 0 {
                        continue;
                    }

                    let dist_x = x.min(sx - 1 - x);
                    let dist_z = z.min(sz - 1 - z);
                    let edge_dist = dist_x.min(dist_z);

                    let y_offset = match edge_dist {
                        0 => -1.2 * self.block_size, 
                        1 => -0.8 * self.block_size, 
                        2 => -0.4 * self.block_size, 
                        _ => 0.0,                    
                    };

                    let exposed = x == 0 || grid[idx(x - 1, y, z)] == 0
                        || x + 1 == sx || grid[idx(x + 1, y, z)] == 0
                        || y == 0 || grid[idx(x, y - 1, z)] == 0
                        || y + 1 == sy || grid[idx(x, y + 1, z)] == 0
                        || z == 0 || grid[idx(x, y, z - 1)] == 0
                        || z + 1 == sz || grid[idx(x, y, z + 1)] == 0;

                    if exposed {
                        let center = Vec3A::new(
                            (x as f32 - sx as f32 * 0.5) * self.block_size,
                            y as f32 * self.block_size + y_offset,
                            (z as f32 - sz as f32 * 0.5) * self.block_size,
                        );

                        cubes.push(Cube::new(center, self.block_size, material));
                    }
                }
            }
        }

        cubes
    }
}
