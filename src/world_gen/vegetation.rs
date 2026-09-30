use super::{
    terrain::index,
    terrain_shape::{pond_at, terrain_at},
    tree::TreePrefab,
};

const STEP: i32 = 5;

#[allow(clippy::too_many_arguments)]
pub(super) fn stamp_trees(
    tree: &TreePrefab,
    grid: &mut [u8],
    sx: usize,
    sy: usize,
    sz: usize,
    offset_x: i32,
    offset_z: i32,
    house_x: i32,
    house_z: i32,
) {
    let start_x = aligned(offset_x - 3);
    let start_z = aligned(offset_z - 3);
    for wx in (start_x..=offset_x + sx as i32 + 3).step_by(STEP as usize) {
        for wz in (start_z..=offset_z + sz as i32 + 3).step_by(STEP as usize) {
            let roll = (((wx as u32).wrapping_mul(73856093) ^ (wz as u32).wrapping_mul(19349663))
                .wrapping_mul(83492791))
                % 100;
            if roll >= 30 || blocked(wx, wz, house_x, house_z) {
                continue;
            }
            let lx = wx - offset_x;
            let lz = wz - offset_z;
            let ground = if lx >= 0 && lx < sx as i32 && lz >= 0 && lz < sz as i32 {
                grid_ground(grid, lx as usize, lz as usize, sx, sy, sz)
            } else {
                let shape = terrain_at(wx as f32, wz as f32);
                (!pond_at(wx as f32, wz as f32) && !shape.is_spire && shape.plateau_height <= 2)
                    .then_some(shape.plateau_height)
            };
            if let Some(y) = ground.filter(|y| y + 7 < sy) {
                tree.stamp_at_world_pos(grid, sx, sy, sz, wx, y + 1, wz, offset_x, offset_z);
            }
        }
    }
}

fn aligned(value: i32) -> i32 {
    value + (STEP - value.rem_euclid(STEP)).rem_euclid(STEP)
}

fn grid_ground(grid: &[u8], x: usize, z: usize, sx: usize, sy: usize, sz: usize) -> Option<usize> {
    for y in (0..sy).rev() {
        match grid[index(x, y, z, sx, sz)] {
            1 => return (y <= 2).then_some(y),
            0 => {}
            _ => return None,
        }
    }
    None
}

fn blocked(x: i32, z: i32, hx: i32, hz: i32) -> bool {
    let house = x >= hx - 1 && x <= hx + 10 && z >= hz - 1 && z <= hz + 10;
    let corridor = x >= hx - 2 && x <= hx + 11 && z > hz + 10 && z <= hz + 25;
    let spire = (x as f32 - 25.0).powi(2) + (z as f32 - 23.0).powi(2) < 18.0;
    house || corridor || spire
}
