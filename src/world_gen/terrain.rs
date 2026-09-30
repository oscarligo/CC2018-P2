use crate::vector::Vec3A;
use crate::cube::Cube;
use crate::material::Material;
use crate::world_gen::house::HousePrefab;
use crate::world_gen::tree::TreePrefab;

pub const PLATEAU_FREQ: f32 = 0.08;
pub const PLATEAU_THRESHOLD: f32 = 0.35;
pub const PLATEAU_HEIGHT_BONUS: usize = 1;

pub const SPIRE_FREQ_COARSE: f32 = 0.065; 
pub const SPIRE_FREQ_FINE: f32 = 0.045;
pub const SPIRE_EXPONENT: i32 = 16;
pub const SPIRE_DETECTION_THRESHOLD: f32 = 0.32;
pub const SPIRE_HEIGHT_PEAK: usize = 9;
pub const SPIRE_HEIGHT_MID: usize = 6;
pub const SPIRE_HEIGHT_BASE: usize = 3;

pub const PEAK_REGION_FREQ: f32 = 0.05;
pub const PEAK_REGION_CHANCE_THRESHOLD: f32 = 0.40;
pub const PEAK_SHARPNESS_EXP_1: i32 = 10;
pub const PEAK_SHARPNESS_EXP_2: i32 = 12;
pub const PEAK_HEIGHT_HIGH: usize = 3;
pub const PEAK_HEIGHT_MED: usize = 2;
pub const PEAK_HEIGHT_LOW: usize = 1;

pub const POND_CELL_SIZE: f32 = 64.0;
pub const POND_BASE_RADIUS: f32 = 6.0;
pub const POND_RADIUS_VARIATION: f32 = 1.5;
pub const POND_WATER_Y_OFFSET: f32 = -0.35;

pub const HOUSE_ORIGIN_X: i32 = 11;
pub const HOUSE_ORIGIN_Z: i32 = 11;
pub const HOUSE_BASE_Y: usize = 1;

pub const INITIAL_SPIRE_X: f32 = 25.0;
pub const INITIAL_SPIRE_Z: f32 = 23.0;

pub const INITIAL_POND_X: f32 = 5.0;
pub const INITIAL_POND_Z: f32 = 22.0;
pub const INITIAL_POND_RADIUS: f32 = 4.5;

pub const TREE_STEP_GRID: i32 = 5;  
pub const TREE_CHANCE_PERCENT: u32 = 30; 

pub const EDGE_SINK_LAYER_0: f32 = -1.2;
pub const EDGE_SINK_LAYER_1: f32 = -0.8;
pub const EDGE_SINK_LAYER_2: f32 = -0.4;

pub struct WorldGenerator {
    pub size_x: usize,
    pub size_y: usize,
    pub size_z: usize,
    pub block_size: f32,
    pub offset_x: i32,
    pub offset_z: i32,
    house: HousePrefab,
    tree: TreePrefab,
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
            house: HousePrefab::new(),
            tree: TreePrefab::new(),
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
        wall_mat: Material,
        roof_mat: Material,
        glass_mat: Material,
        ext_floor_mat: Material,
        int_floor_mat: Material,
        int_detail_mat: Material,
        pillar_mat: Material,
        trunk_mat: Material,
        leaves_mat: Material,
        light_mat: Material,
    ) -> Vec<Cube> {
        let total = self.size_x * self.size_y * self.size_z;
        let mut grid = vec![0u8; total];

        let idx = |x: usize, y: usize, z: usize| -> usize {
            (y * self.size_z + z) * self.size_x + x
        };

        for x in 0..self.size_x {
            let world_x = (x as i32 + self.offset_x) as f32;

            for z in 0..self.size_z {
                let world_z = (z as i32 + self.offset_z) as f32;

                let plateau_wave = ((world_x * PLATEAU_FREQ).sin() + (world_z * PLATEAU_FREQ).cos()) * 0.5;
                let plateau_extra = if plateau_wave > PLATEAU_THRESHOLD { PLATEAU_HEIGHT_BONUS } else { 0 };

                let s_rare = (world_x * SPIRE_FREQ_COARSE + (world_z * SPIRE_FREQ_FINE).cos() * 2.0).sin() * 0.5 + 0.5;
                let c_rare = (world_z * SPIRE_FREQ_COARSE + (world_x * SPIRE_FREQ_FINE).sin() * 2.0).cos() * 0.5 + 0.5;
                let spire_factor = (s_rare * c_rare).powi(SPIRE_EXPONENT);

                let dist_init_spire_sq = (world_x - INITIAL_SPIRE_X).powi(2) + (world_z - INITIAL_SPIRE_Z).powi(2);
                let is_initial_spire = dist_init_spire_sq < 14.0;

                let is_rare_spire = is_initial_spire || spire_factor > SPIRE_DETECTION_THRESHOLD;

                let peak_extra = if is_initial_spire {
                    if dist_init_spire_sq < 2.0 {
                        9
                    } else if dist_init_spire_sq < 6.0 {
                        6
                    } else {
                        3
                    }
                } else if spire_factor > 0.72 {
                    SPIRE_HEIGHT_PEAK
                } else if spire_factor > 0.50 {
                    SPIRE_HEIGHT_MID
                } else if spire_factor > SPIRE_DETECTION_THRESHOLD {
                    SPIRE_HEIGHT_BASE
                } else {
                    let region_mask = ((world_x * PEAK_REGION_FREQ).sin() * (world_z * PEAK_REGION_FREQ).cos()).abs();
                    if region_mask > PEAK_REGION_CHANCE_THRESHOLD {
                        let w1 = (world_x * 0.28 + (world_z * 0.15).sin() * 1.2).sin() * 0.5 + 0.5;
                        let w2 = (world_z * 0.31 + (world_x * 0.13).cos() * 1.2).cos() * 0.5 + 0.5;
                        let p1 = (w1 * w2).powi(PEAK_SHARPNESS_EXP_1);

                        let w3 = (world_x * 0.42 - world_z * 0.25).sin() * 0.5 + 0.5;
                        let w4 = (world_z * 0.39 + world_x * 0.21).cos() * 0.5 + 0.5;
                        let p2 = (w3 * w4).powi(PEAK_SHARPNESS_EXP_2);

                        let combined = p1.max(p2);

                        if combined > 0.70 {
                            PEAK_HEIGHT_HIGH
                        } else if combined > 0.48 {
                            PEAK_HEIGHT_MED
                        } else if combined > 0.30 {
                            PEAK_HEIGHT_LOW
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                };

                let local_px = (world_x.rem_euclid(POND_CELL_SIZE)) - (POND_CELL_SIZE * 0.5);
                let local_pz = (world_z.rem_euclid(POND_CELL_SIZE)) - (POND_CELL_SIZE * 0.5);
                let pond_dist_sq = local_px * local_px + local_pz * local_pz;

                let angle = local_pz.atan2(local_px);
                let pond_radius = POND_BASE_RADIUS + POND_RADIUS_VARIATION * (angle * 3.0).sin();
                let is_procedural_pond = pond_dist_sq < (pond_radius * pond_radius) && peak_extra == 0;

                let dx_init_pond = world_x - INITIAL_POND_X;
                let dz_init_pond = world_z - INITIAL_POND_Z;
                let angle_init = dz_init_pond.atan2(dx_init_pond);
                let r_init_mod = INITIAL_POND_RADIUS + 0.8 * (angle_init * 2.0).sin();
                let is_initial_pond = (dx_init_pond * dx_init_pond + dz_init_pond * dz_init_pond) < (r_init_mod * r_init_mod);

                let is_in_pond = (is_initial_pond || is_procedural_pond) && peak_extra == 0;

                if is_in_pond {
                    grid[idx(x, 0, z)] = 2; // Agua
                } else {
                    let extra_height = plateau_extra + peak_extra;
                    let max_y = extra_height.min(self.size_y.saturating_sub(1));

                    for y in 0..=max_y {
                        let block_type = if y > plateau_extra && !is_rare_spire {
                            3 // Pico pequeño
                        } else {
                            1 // Terreno / Pico grande
                        };
                        grid[idx(x, y, z)] = block_type;
                    }
                }
            }
        }

        self.house.stamp_at_world_pos(
            &mut grid,
            self.size_x,
            self.size_y,
            self.size_z,
            HOUSE_ORIGIN_X,
            HOUSE_BASE_Y,
            HOUSE_ORIGIN_Z,
            self.offset_x,
            self.offset_z,
        );

        let tree_margin = 3i32;
        let min_wx = self.offset_x - tree_margin;
        let max_wx = self.offset_x + self.size_x as i32 + tree_margin;
        let min_wz = self.offset_z - tree_margin;
        let max_wz = self.offset_z + self.size_z as i32 + tree_margin;

        let start_wx = min_wx + (TREE_STEP_GRID - (min_wx.rem_euclid(TREE_STEP_GRID))).rem_euclid(TREE_STEP_GRID);
        let start_wz = min_wz + (TREE_STEP_GRID - (min_wz.rem_euclid(TREE_STEP_GRID))).rem_euclid(TREE_STEP_GRID);

        let mut wx = start_wx;
        while wx <= max_wx {
            let mut wz = start_wz;
            while wz <= max_wz {
                let local_x = wx - self.offset_x;
                let local_z = wz - self.offset_z;

                let seed = (((wx as u32).wrapping_mul(73856093)) ^ ((wz as u32).wrapping_mul(19349663)))
                    .wrapping_mul(83492791);
                let roll = seed % 100;

                let in_house_footprint = wx >= HOUSE_ORIGIN_X - 1 && wx <= HOUSE_ORIGIN_X + 10
                    && wz >= HOUSE_ORIGIN_Z - 1 && wz <= HOUSE_ORIGIN_Z + 10;

                let in_front_corridor = wx >= HOUSE_ORIGIN_X - 2 && wx <= HOUSE_ORIGIN_X + 11
                    && wz > HOUSE_ORIGIN_Z + 10 && wz <= HOUSE_ORIGIN_Z + 10 + 15;

                let dist_to_spire_sq = (wx as f32 - INITIAL_SPIRE_X).powi(2) + (wz as f32 - INITIAL_SPIRE_Z).powi(2);
                let on_spire_area = dist_to_spire_sq < 18.0;

                let blocked = in_house_footprint || in_front_corridor || on_spire_area;

                if roll < TREE_CHANCE_PERCENT && !blocked {
                    let ground_y = if local_x >= 0 && local_x < self.size_x as i32 && local_z >= 0 && local_z < self.size_z as i32 {
                        let mut found_y = None;
                        for y in (0..self.size_y).rev() {
                            let block = grid[idx(local_x as usize, y, local_z as usize)];
                            if block == 1 {
                                if y <= 2 {
                                    found_y = Some(y);
                                }
                                break;
                            } else if block != 0 {
                                break;
                            }
                        }
                        found_y
                    } else {
                        let world_x = wx as f32;
                        let world_z = wz as f32;

                        let plateau_wave = ((world_x * PLATEAU_FREQ).sin() + (world_z * PLATEAU_FREQ).cos()) * 0.5;
                        let plateau_extra = if plateau_wave > PLATEAU_THRESHOLD { PLATEAU_HEIGHT_BONUS } else { 0 };

                        let s_rare = (world_x * SPIRE_FREQ_COARSE + (world_z * SPIRE_FREQ_FINE).cos() * 2.0).sin() * 0.5 + 0.5;
                        let c_rare = (world_z * SPIRE_FREQ_COARSE + (world_x * SPIRE_FREQ_FINE).sin() * 2.0).cos() * 0.5 + 0.5;
                        let spire_factor = (s_rare * c_rare).powi(SPIRE_EXPONENT);
                        let is_init_spire = ((world_x - INITIAL_SPIRE_X).powi(2) + (world_z - INITIAL_SPIRE_Z).powi(2)) < 14.0;
                        let is_rare_spire = is_init_spire || spire_factor > SPIRE_DETECTION_THRESHOLD;

                        let local_px = (world_x.rem_euclid(POND_CELL_SIZE)) - (POND_CELL_SIZE * 0.5);
                        let local_pz = (world_z.rem_euclid(POND_CELL_SIZE)) - (POND_CELL_SIZE * 0.5);
                        let pond_dist_sq = local_px * local_px + local_pz * local_pz;
                        let angle = local_pz.atan2(local_px);
                        let pond_radius = POND_BASE_RADIUS + POND_RADIUS_VARIATION * (angle * 3.0).sin();

                        let dx_init_pond = world_x - INITIAL_POND_X;
                        let dz_init_pond = world_z - INITIAL_POND_Z;
                        let angle_init = dz_init_pond.atan2(dx_init_pond);
                        let r_init_mod = INITIAL_POND_RADIUS + 0.8 * (angle_init * 2.0).sin();
                        let in_pond = (dx_init_pond * dx_init_pond + dz_init_pond * dz_init_pond) < (r_init_mod * r_init_mod)
                            || pond_dist_sq < (pond_radius * pond_radius);

                        if !in_pond && !is_rare_spire && plateau_extra <= 2 {
                            Some(plateau_extra)
                        } else {
                            None
                        }
                    };

                    if let Some(gy) = ground_y {
                        if gy + 7 < self.size_y {
                            self.tree.stamp_at_world_pos(
                                &mut grid,
                                self.size_x,
                                self.size_y,
                                self.size_z,
                                wx,
                                gy + 1,
                                wz,
                                self.offset_x,
                                self.offset_z,
                            );
                        }
                    }
                }

                wz += TREE_STEP_GRID;
            }
            wx += TREE_STEP_GRID;
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
                        0 => EDGE_SINK_LAYER_0 * self.block_size,
                        1 => EDGE_SINK_LAYER_1 * self.block_size,
                        2 => EDGE_SINK_LAYER_2 * self.block_size,
                        _ => 0.0,
                    };

                    let water_sink_offset = if cell_type == 2 {
                        POND_WATER_Y_OFFSET * self.block_size
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
                            4 => wall_mat,
                            5 => roof_mat,
                            6 => glass_mat,
                            7 => ext_floor_mat, 
                            8 => int_floor_mat,
                            9 => int_detail_mat,
                            10 => pillar_mat,
                            11 => trunk_mat,
                            12 => leaves_mat,
                            13 => light_mat,
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
