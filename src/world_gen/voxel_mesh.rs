use super::terrain::index;
use crate::{cube::Cube, material::Material, vector::Vec3A};

pub(super) struct Materials {
    pub terrain: Material,
    pub water: Material,
    pub peak: Material,
    pub wall: Material,
    pub roof: Material,
    pub glass: Material,
    pub ext_floor: Material,
    pub int_floor: Material,
    pub int_detail: Material,
    pub pillar: Material,
    pub trunk: Material,
    pub leaves: Material,
    pub light: Material,
}

pub(super) fn visible_cubes(
    grid: &[u8],
    sx: usize,
    sy: usize,
    sz: usize,
    block_size: f32,
    materials: Materials,
) -> Vec<Cube> {
    let mut cubes = Vec::new();
    for y in 0..sy {
        for z in 0..sz {
            for x in 0..sx {
                let cell = grid[index(x, y, z, sx, sz)];
                if cell == 0 || !exposed(grid, x, y, z, sx, sy, sz, cell) {
                    continue;
                }
                let edge = x.min(sx - 1 - x).min(z.min(sz - 1 - z));
                let edge_offset = [-1.2, -0.8, -0.4].get(edge).copied().unwrap_or(0.0) * block_size;
                let water_offset = if cell == 2 { -0.35 * block_size } else { 0.0 };
                let center = Vec3A::new(
                    (x as f32 - sx as f32 * 0.5) * block_size,
                    y as f32 * block_size + edge_offset + water_offset,
                    (z as f32 - sz as f32 * 0.5) * block_size,
                );
                cubes.push(Cube::new(center, block_size, materials.get(cell)));
            }
        }
    }
    cubes
}

fn exposed(
    g: &[u8],
    x: usize,
    y: usize,
    z: usize,
    sx: usize,
    sy: usize,
    sz: usize,
    cell: u8,
) -> bool {
    x == 0
        || g[index(x - 1, y, z, sx, sz)] == 0
        || x + 1 == sx
        || g[index(x + 1, y, z, sx, sz)] == 0
        || y == 0
        || g[index(x, y - 1, z, sx, sz)] == 0
        || y + 1 == sy
        || g[index(x, y + 1, z, sx, sz)] == 0
        || z == 0
        || g[index(x, y, z - 1, sx, sz)] == 0
        || z + 1 == sz
        || g[index(x, y, z + 1, sx, sz)] == 0
        || (cell != 2
            && ((x > 0 && g[index(x - 1, y, z, sx, sz)] == 2)
                || (x + 1 < sx && g[index(x + 1, y, z, sx, sz)] == 2)
                || (z > 0 && g[index(x, y, z - 1, sx, sz)] == 2)
                || (z + 1 < sz && g[index(x, y, z + 1, sx, sz)] == 2)))
}

impl Materials {
    fn get(&self, cell: u8) -> Material {
        match cell {
            2 => self.water,
            3 => self.peak,
            4 => self.wall,
            5 => self.roof,
            6 => self.glass,
            7 => self.ext_floor,
            8 => self.int_floor,
            9 => self.int_detail,
            10 => self.pillar,
            11 => self.trunk,
            12 => self.leaves,
            13 => self.light,
            _ => self.terrain,
        }
    }
}
