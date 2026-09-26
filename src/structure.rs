pub struct HousePrefab {
    pub blocks: Vec<(usize, usize, usize, u8)>,
}

impl HousePrefab {
    pub fn new() -> Self {
        use std::collections::BTreeMap;

        const W: usize = 10;
        const D: usize = 10;
        const WOOD: u8 = 4;
        const ROOF: u8 = 5;
        const GLASS: u8 = 6;
        const EXT_FLOOR: u8 = 7; // Suelo exterior (porche)
        const INT_FLOOR: u8 = 8; // Suelo interior de la casa
        const INT_DETAIL: u8 = 9; // Detalles interiores (paredes, escaleras, etc.)

        let mut voxels = BTreeMap::new();
        let mut put = |x: usize, y: usize, z: usize, id: u8| {
            voxels.insert((x, y, z), id);
        };

        // Suelo de planta baja (y=0) y segundo piso (y=4) con material interior
        for x in 1..=8 {
            for z in 2..=8 {
                put(x, 0, z, INT_FLOOR);
                let stairwell = (6..=7).contains(&x) && (3..=6).contains(&z);
                if !stairwell {
                    put(x, 4, z, INT_FLOOR);
                }
            }
        }

        // Paredes y ventanas
        for x in 1..=8 {
            for z in 2..=8 {
                if x != 1 && x != 8 && z != 2 && z != 8 {
                    continue;
                }
                for y in 1..=7 {
                    if z == 2 && (4..=5).contains(&x) && y <= 3 {
                        continue;
                    }
                    let window_height = (2..=3).contains(&y) || (6..=7).contains(&y);
                    let side = (x == 1 || x == 8) && (4..=5).contains(&z);
                    let front = z == 2 && ((2..=3).contains(&x) || (6..=7).contains(&x));
                    let back = z == 8 && (3..=6).contains(&x);
                    let window = window_height && (side || front || back);
                    put(x, y, z, if window { GLASS } else { WOOD });
                }
            }
        }

        // Techo
        for x in 0..W {
            let roof_y = 8 + x.min(W - 1 - x) / 2;
            for z in 1..D {
                put(x, roof_y, z, ROOF);
            }
            if (1..=8).contains(&x) {
                for y in 8..roof_y {
                    put(x, y, 2, WOOD);
                    put(x, y, 8, WOOD);
                }
            }
        }

        // Porche exterior: piso exterior y tejadillo
        for x in 2..=7 {
            for z in 0..=1 {
                put(x, 0, z, EXT_FLOOR);
                put(x, 4, z, ROOF);
            }
        }
        for &x in &[2, 7] {
            for y in 1..=3 {
                put(x, y, 0, WOOD);
            }
            put(x, 1, 1, WOOD);
        }

        // Escalera
        for step in 0..4 {
            for x in 6..=7 {
                for y in 1..=(step + 1) {
                    put(x, y, 3 + step, INT_FLOOR);
                }
            }
        }

        // Muro divisorio y detalles interiores
        for z in 3..=6 {
            put(5, 5, z, INT_DETAIL);
        }
        for x in 2..=3 {
            put(x, 1, 4, INT_DETAIL);
        }
        put(2, 1, 3, INT_DETAIL);
        for x in 2..=3 {
            for z in 5..=6 {
                put(x, 5, z, INT_DETAIL);
            }
        }

        let blocks = voxels
            .into_iter()
            .map(|((x, y, z), id)| (W - 1 - x, y, D - 1 - z, id))
            .collect();

        Self { blocks }
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
            let target_wx = world_gx + lx as i32;
            let target_wz = world_gz + lz as i32;
            let local_x = target_wx - offset_x;
            let local_z = target_wz - offset_z;
            let local_y = world_gy + ly;

            if local_x >= 0 && local_x < sx as i32
                && local_z >= 0 && local_z < sz as i32
                && local_y < sy
            {
                grid[idx(local_x as usize, local_y, local_z as usize)] = block_id;
            }
        }
    }
}
