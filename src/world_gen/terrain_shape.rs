const PLATEAU_FREQ: f32 = 0.08;
const SPIRE_COARSE: f32 = 0.065;
const SPIRE_FINE: f32 = 0.045;
const SPIRE_THRESHOLD: f32 = 0.32;
const INITIAL_SPIRE: (f32, f32) = (25.0, 23.0);
const INITIAL_POND: (f32, f32) = (5.0, 22.0);

pub(super) struct Terrain {
    pub plateau_height: usize,
    pub peak_height: usize,
    pub is_spire: bool,
    pub is_pond: bool,
}

pub(super) fn terrain_at(x: f32, z: f32) -> Terrain {
    let plateau_height =
        usize::from(((x * PLATEAU_FREQ).sin() + (z * PLATEAU_FREQ).cos()) * 0.5 > 0.35);
    let spire = spire_factor(x, z);
    let initial_distance = (x - INITIAL_SPIRE.0).powi(2) + (z - INITIAL_SPIRE.1).powi(2);
    let initial = initial_distance < 14.0;
    let is_spire = initial || spire > SPIRE_THRESHOLD;
    let peak_height = if initial {
        if initial_distance < 2.0 {
            9
        } else if initial_distance < 6.0 {
            6
        } else {
            3
        }
    } else if spire > 0.72 {
        9
    } else if spire > 0.50 {
        6
    } else if spire > SPIRE_THRESHOLD {
        3
    } else {
        small_peak(x, z)
    };
    Terrain {
        plateau_height,
        peak_height,
        is_spire,
        is_pond: peak_height == 0 && pond_at(x, z),
    }
}

pub(super) fn pond_at(x: f32, z: f32) -> bool {
    let local_x = x.rem_euclid(64.0) - 32.0;
    let local_z = z.rem_euclid(64.0) - 32.0;
    let radius = 6.0 + 1.5 * (local_z.atan2(local_x) * 3.0).sin();
    let procedural = local_x * local_x + local_z * local_z < radius * radius;
    let dx = x - INITIAL_POND.0;
    let dz = z - INITIAL_POND.1;
    let radius = 4.5 + 0.8 * (dz.atan2(dx) * 2.0).sin();
    procedural || dx * dx + dz * dz < radius * radius
}

fn spire_factor(x: f32, z: f32) -> f32 {
    let a = (x * SPIRE_COARSE + (z * SPIRE_FINE).cos() * 2.0).sin() * 0.5 + 0.5;
    let b = (z * SPIRE_COARSE + (x * SPIRE_FINE).sin() * 2.0).cos() * 0.5 + 0.5;
    (a * b).powi(16)
}

fn small_peak(x: f32, z: f32) -> usize {
    if ((x * 0.05).sin() * (z * 0.05).cos()).abs() <= 0.40 {
        return 0;
    }
    let a = (((x * 0.28 + (z * 0.15).sin() * 1.2).sin() * 0.5 + 0.5)
        * ((z * 0.31 + (x * 0.13).cos() * 1.2).cos() * 0.5 + 0.5))
        .powi(10);
    let b = (((x * 0.42 - z * 0.25).sin() * 0.5 + 0.5) * ((z * 0.39 + x * 0.21).cos() * 0.5 + 0.5))
        .powi(12);
    match a.max(b) {
        value if value > 0.70 => 3,
        value if value > 0.48 => 2,
        value if value > 0.30 => 1,
        _ => 0,
    }
}