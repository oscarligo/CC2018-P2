pub struct TreePrefab {
    pub blocks: Vec<(usize, usize, usize, u8)>,
}

impl TreePrefab {
    pub fn new() -> Self {
        use std::collections::BTreeMap;

        const TRUNK: u8 = 11;
        const LEAVES: u8 = 12;

        let mut voxels = BTreeMap::new();
        let mut put = |x: usize, y: usize, z: usize, id: u8| {
            voxels.insert((x, y, z), id);
        };

        for y in 0..5 {
            put(2, y, 2, TRUNK);
        }

        for y in 3..=4 {
            for x in 0..=4 {
                for z in 0..=4 {
                    // Quitar las 4 esquinas exteriores para redondear el follaje
                    let is_corner = (x == 0 || x == 4) && (z == 0 || z == 4);
                    if !is_corner && !(x == 2 && z == 2 && y < 5) {
                        put(x, y, z, LEAVES);
                    }
                }
            }
        }

        for x in 1..=3 {
            for z in 1..=3 {
                let is_corner = (x == 1 || x == 3) && (z == 1 || z == 3);
                if !is_corner {
                    put(x, 5, z, LEAVES);
                }
            }
        }

        put(2, 6, 2, LEAVES);
        put(1, 6, 2, LEAVES);
        put(3, 6, 2, LEAVES);
        put(2, 6, 1, LEAVES);
        put(2, 6, 3, LEAVES);

        let blocks = voxels
            .into_iter()
            .map(|((x, y, z), id)| (x, y, z, id))
            .collect();

        Self {
            blocks,
        }
    }

    pub fn stamp_at_world_pos(
        &self,
        grid: &mut [u8],
        sx: usize,
        sy: usize,
        sz: usize,
        world_gx: i32,
        world_gy: usize,
        world_gz: i32,
        offset_x: i32,
        offset_z: i32,
    ) {
        let idx = |x: usize, y: usize, z: usize| (y * sz + z) * sx + x;

        for &(lx, ly, lz, block_id) in &self.blocks {
            // El origen local (2, 0, 2) corresponde a la base del tronco
            let target_wx = world_gx + lx as i32 - 2;
            let target_wz = world_gz + lz as i32 - 2;
            let local_x = target_wx - offset_x;
            let local_z = target_wz - offset_z;
            let local_y = world_gy + ly;

            if local_x >= 0
                && local_x < sx as i32
                && local_z >= 0
                && local_z < sz as i32
                && local_y < sy
            {
                let index = idx(local_x as usize, local_y, local_z as usize);
                // No sobreescribir bloques sólidos existentes (por ejemplo, estructuras o rocas)
                if grid[index] == 0 {
                    grid[index] = block_id;
                }
            }
        }
    }
}
