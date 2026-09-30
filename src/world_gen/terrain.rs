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
        let mut grid = vec![0; self.size_x * self.size_y * self.size_z];
        for x in 0..self.size_x {
            for z in 0..self.size_z {
                let shape = terrain_at(
                    (x as i32 + self.offset_x) as f32,
                    (z as i32 + self.offset_z) as f32,
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
            self.offset_x,
            self.offset_z,
        );
        stamp_trees(
            &self.tree,
            &mut grid,
            self.size_x,
            self.size_y,
            self.size_z,
            self.offset_x,
            self.offset_z,
            HOUSE_X,
            HOUSE_Z,
        );
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
}
