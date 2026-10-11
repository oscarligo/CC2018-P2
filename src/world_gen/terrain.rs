use super::{
    house::HousePrefab,
    terrain_shape::terrain_at,
    tree::TreePrefab,
    vegetation::stamp_trees,
    voxel_mesh::{Materials, visible_cubes},
};
use crate::{cube::Cube, material::Material};

const HOUSE_X: i32 = 11;
const HOUSE_Z: i32 = 11;

pub(super) fn index(x: usize, y: usize, z: usize, sx: usize, sz: usize) -> usize {
    (y * sz + z) * sx + x
}

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

    pub fn center_ground_height(&self) -> f32 {
        let terrain = terrain_at(
            (self.size_x / 2) as f32 + self.offset_x as f32,
            (self.size_z / 2) as f32 + self.offset_z as f32,
        );
        if terrain.is_pond {
            0.15 * self.block_size
        } else {
            (terrain.plateau_height + terrain.peak_height) as f32 * self.block_size
                + self.block_size * 0.5
        }
    }

    pub fn movement_ground(
        &self,
        dx: i32,
        dz: i32,
        feet_y: f32,
        player_height: f32,
    ) -> Option<f32> {
        let grid = self.build_grid(self.offset_x + dx, self.offset_z + dz);
        let x = self.size_x / 2;
        let z = self.size_z / 2;
        let player_top: f32 = feet_y + player_height;
        let mut support: f32 = 0.0;

        for y in 0..self.size_y {
            let cell = grid[index(x, y, z, self.size_x, self.size_z)];
            if cell == 0 {
                continue;
            }
            let center = y as f32 * self.block_size
                + if cell == 2 {
                    -0.35 * self.block_size
                } else {
                    0.0
                };
            let min = center - self.block_size * 0.5;
            let max = center + self.block_size * 0.5;
            if max <= feet_y + 0.001 {
                support = support.max(max);
            } else if feet_y < max - 0.001 && player_top > min + 0.001 {
                return None;
            }
        }

        Some(support)
    }

    pub fn player_collides(&self, feet_y: f32, player_height: f32) -> bool {
        self.movement_ground(0, 0, feet_y, player_height)
            .is_none()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        &self,
        terrain: Material,
        water: Material,
        peak: Material,
        wall: Material,
        roof: Material,
        glass: Material,
        ext_floor: Material,
        int_floor: Material,
        int_detail: Material,
        pillar: Material,
        trunk: Material,
        leaves: Material,
        light: Material,
    ) -> Vec<Cube> {
        let grid = self.build_grid(self.offset_x, self.offset_z);
        visible_cubes(
            &grid,
            self.size_x,
            self.size_y,
            self.size_z,
            self.block_size,
            Materials {
                terrain,
                water,
                peak,
                wall,
                roof,
                glass,
                ext_floor,
                int_floor,
                int_detail,
                pillar,
                trunk,
                leaves,
                light,
            },
        )
    }

    fn build_grid(&self, offset_x: i32, offset_z: i32) -> Vec<u8> {
        let mut grid = vec![0; self.size_x * self.size_y * self.size_z];
        for x in 0..self.size_x {
            for z in 0..self.size_z {
                let shape = terrain_at(
                    (x as i32 + offset_x) as f32,
                    (z as i32 + offset_z) as f32,
                );
                if shape.is_pond {
                    grid[index(x, 0, z, self.size_x, self.size_z)] = 2;
                    continue;
                }
                let top =
                    (shape.plateau_height + shape.peak_height).min(self.size_y.saturating_sub(1));
                for y in 0..=top {
                    grid[index(x, y, z, self.size_x, self.size_z)] =
                        if y > shape.plateau_height && !shape.is_spire {
                            3
                        } else {
                            1
                        };
                }
            }
        }
        self.house.stamp_at_world_pos(
            &mut grid,
            self.size_x,
            self.size_y,
            self.size_z,
            HOUSE_X,
            1,
            HOUSE_Z,
            offset_x,
            offset_z,
        );
        stamp_trees(
            &self.tree,
            &mut grid,
            self.size_x,
            self.size_y,
            self.size_z,
            offset_x,
            offset_z,
            HOUSE_X,
            HOUSE_Z,
        );
        grid
    }
}
