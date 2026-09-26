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

    pub fn generate(
        &self,
        terrain_mat: Material,
        water_mat: Material,
        peak_mat: Material,
    ) -> Vec<Cube> {
        let total = self.size_x * self.size_y * self.size_z;
        // 0 = Aire, 1 = Terreno/Pico grande, 2 = Agua, 3 = Pico pequeño
        let mut grid = vec![0u8; total];

        let idx = |x: usize, y: usize, z: usize| -> usize {
            (y * self.size_z + z) * self.size_x + x
        };

        for x in 0..self.size_x {
            let world_x = (x as i32 + self.offset_x) as f32;

            for z in 0..self.size_z {
                let world_z = (z as i32 + self.offset_z) as f32;

                let plateau_wave = ((world_x * 0.08).sin() + (world_z * 0.08).cos()) * 0.5;
                let plateau_extra = if plateau_wave > 0.35 { 1 } else { 0 };

                let s_rare = (world_x * 0.065 + (world_z * 0.045).cos() * 2.0).sin() * 0.5 + 0.5;
                let c_rare = (world_z * 0.065 + (world_x * 0.045).sin() * 2.0).cos() * 0.5 + 0.5;
                let spire_factor = (s_rare * c_rare).powi(16);

                let is_rare_spire = spire_factor > 0.32;

                let peak_extra = if spire_factor > 0.72 {
                    9
                } else if spire_factor > 0.50 {
                    6
                } else if spire_factor > 0.32 {
                    3
                } else {
                    let region_mask = ((world_x * 0.05).sin() * (world_z * 0.05).cos()).abs();
                    if region_mask > 0.40 {
                        let w1 = (world_x * 0.28 + (world_z * 0.15).sin() * 1.2).sin() * 0.5 + 0.5;
                        let w2 = (world_z * 0.31 + (world_x * 0.13).cos() * 1.2).cos() * 0.5 + 0.5;
                        let p1 = (w1 * w2).powi(10);

                        let w3 = (world_x * 0.42 - world_z * 0.25).sin() * 0.5 + 0.5;
                        let w4 = (world_z * 0.39 + world_x * 0.21).cos() * 0.5 + 0.5;
                        let p2 = (w3 * w4).powi(12);

                        let combined = p1.max(p2);

                        if combined > 0.70 {
                            3
                        } else if combined > 0.48 {
                            2
                        } else if combined > 0.30 {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                };

                let pond_cell_size = 64.0;
                let local_px = (world_x.rem_euclid(pond_cell_size)) - (pond_cell_size * 0.5);
                let local_pz = (world_z.rem_euclid(pond_cell_size)) - (pond_cell_size * 0.5);
                let pond_dist_sq = local_px * local_px + local_pz * local_pz;

                let angle = local_pz.atan2(local_px);
                let pond_radius = 6.0 + 1.5 * (angle * 3.0).sin();
                let is_in_pond = pond_dist_sq < (pond_radius * pond_radius) && peak_extra == 0;

                if is_in_pond {
                    grid[idx(x, 0, z)] = 2;
                } else {
                    let extra_height = plateau_extra + peak_extra;
                    let max_y = extra_height.min(self.size_y.saturating_sub(1));

                    for y in 0..=max_y {
                        // Solo los picos pequeños (no raros) usan peak_mat; los picos grandes usan terrain_mat
                        let block_type = if y > plateau_extra && !is_rare_spire {
                            3
                        } else {
                            1
                        };
                        grid[idx(x, y, z)] = block_type;
                    }
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
                    let cell_type = grid[idx(x, y, z)];
                    if cell_type == 0 {
                        continue;
                    }

                    let dist_x = x.min(sx - 1 - x);
                    let dist_z = z.min(sz - 1 - z);
                    let edge_dist = dist_x.min(dist_z);

                    let edge_y_offset = match edge_dist {
                        0 => -1.2 * self.block_size,
                        1 => -0.8 * self.block_size,
                        2 => -0.4 * self.block_size,
                        _ => 0.0,
                    };

                    let water_sink_offset = if cell_type == 2 {
                        -0.35 * self.block_size
                    } else {
                        0.0
                    };

                    let exposed = x == 0 || grid[idx(x - 1, y, z)] == 0
                        || x + 1 == sx || grid[idx(x + 1, y, z)] == 0
                        || y == 0 || grid[idx(x, y - 1, z)] == 0
                        || y + 1 == sy || grid[idx(x, y + 1, z)] == 0
                        || z == 0 || grid[idx(x, y, z - 1)] == 0
                        || z + 1 == sz || grid[idx(x, y, z + 1)] == 0
                        || (cell_type != 2 && (
                            (x > 0 && grid[idx(x - 1, y, z)] == 2) ||
                            (x + 1 < sx && grid[idx(x + 1, y, z)] == 2) ||
                            (z > 0 && grid[idx(x, y, z - 1)] == 2) ||
                            (z + 1 < sz && grid[idx(x, y, z + 1)] == 2)
                        ));

                    if exposed {
                        let center = Vec3A::new(
                            (x as f32 - sx as f32 * 0.5) * self.block_size,
                            y as f32 * self.block_size + edge_y_offset + water_sink_offset,
                            (z as f32 - sz as f32 * 0.5) * self.block_size,
                        );

                        let mat = match cell_type {
                            2 => water_mat,
                            3 => peak_mat,
                            _ => terrain_mat,
                        };

                        cubes.push(Cube::new(center, self.block_size, mat));
                    }
                }
            }
        }

        cubes
    }
}
