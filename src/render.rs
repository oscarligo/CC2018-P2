use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::*;
use crate::caster::cast_ray;
use crate::texture_manager::TextureManager;
use crate::enemy::Enemy;
use crate::portal::Portal;
use crate::projectile::Projectile;
use std::f32::consts::PI;


pub fn render_player(framebuffer: &mut Framebuffer, player: &Player) {
    framebuffer.set_current_color(Color::BLUE);
    framebuffer.set_pixel(player.position.x as u32, player.position.y as u32);
}

pub fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Maze,
    block_size: usize,
    world: usize,
    texture_manager: &TextureManager,
) -> Vec<f32> {
    let num_rays = framebuffer.width;
    let height = framebuffer.height as f32;
    let half_height = height / 2.0;
    let mut depth_buffer = vec![f32::INFINITY; num_rays as usize];
    render_floor_and_ceiling(framebuffer, player, block_size, world, texture_manager);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle_offset = (current_ray - 0.5) * player.fov;
        let intersect = cast_ray(framebuffer, player, maze, block_size, angle_offset, false,0.5);

        let corrected_distance = intersect.distance * angle_offset.cos();
        let distance_to_wall = corrected_distance.max(0.1);
        depth_buffer[i as usize] = distance_to_wall;

        let stake_height = (block_size as f32 * height) / distance_to_wall;
        let stake_top_raw = half_height - stake_height / 2.0;
        let stake_top = stake_top_raw.max(0.0).min(height) as u32;
        let stake_bottom = (half_height + stake_height / 2.0).max(0.0).min(height) as u32;
        let u = texture_u(
            intersect.hit_x,
            intersect.hit_y,
            intersect.vertical_hit,
            block_size as f32,
        );

        for y in stake_top..stake_bottom {
            let v = (y as f32 - stake_top_raw) / stake_height;
            let color = texture_manager.get_pixel_color(intersect.impact, u, v);
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(i as u32, y);
        }
    }
    depth_buffer
}

fn render_floor_and_ceiling(
    framebuffer: &mut Framebuffer,
    player: &Player,
    block_size: usize,
    world: usize,
    texture_manager: &TextureManager,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    for y in 0..framebuffer.height {
        if y == framebuffer.height / 2 {
            continue;
        }
        let perpendicular_distance = plane_distance(y as f32, height, block_size as f32);

        for x in 0..framebuffer.width {
            let angle_offset = (x as f32 / width - 0.5) * player.fov;
            let distance = perpendicular_distance / angle_offset.cos();
            let ray_angle = player.angle + angle_offset;
            let world_x = player.position.x + distance * ray_angle.cos();
            let world_y = player.position.y + distance * ray_angle.sin();
            let u = world_x.rem_euclid(block_size as f32) / block_size as f32;
            let v = world_y.rem_euclid(block_size as f32) / block_size as f32;

            let color = if y < framebuffer.height / 2 {
                texture_manager.get_ceiling_pixel(world, u, v)
            } else {
                texture_manager.get_floor_pixel(world, u, v)
            };
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(x, y);
        }
    }
}

fn plane_distance(screen_y: f32, screen_height: f32, block_size: f32) -> f32 {
    block_size * screen_height / (2.0 * (screen_y - screen_height / 2.0).abs())
}

fn texture_u(hit_x: f32, hit_y: f32, vertical_hit: bool, block_size: f32) -> f32 {
    let offset = if vertical_hit { hit_y } else { hit_x };
    offset.rem_euclid(block_size) / block_size
}

pub fn render_sprites(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemies: &[Enemy],
    portals: &[Portal],
    projectiles: &[Projectile],
    block_size: usize,
    depth_buffer: &[f32],
    texture_manager: &TextureManager,
) {
    let mut sprites: Vec<_> = enemies
        .iter()
        .map(|enemy| (enemy.position, enemy.marker, enemy.frame(), 1.0))
        .chain(
            portals
                .iter()
                .map(|portal| (portal.position, portal.marker, portal.frame(), 1.0)),
        )
        .chain(
            projectiles
                .iter()
                .map(|projectile| (projectile.position, projectile.marker, 0, 0.2)),
        )
        .collect();
    sprites.sort_by(|a, b| {
        distance_squared(b.0, player.position)
            .total_cmp(&distance_squared(a.0, player.position))
    });

    for (position, marker, frame, scale) in sprites {
        let dx = position.x - player.position.x;
        let dy = position.y - player.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let relative_angle = (dy.atan2(dx) - player.angle + PI).rem_euclid(2.0 * PI) - PI;

        if relative_angle.abs() > player.fov / 2.0 {
            continue;
        }

        let depth = distance * relative_angle.cos();
        if depth <= 0.0 {
            continue;
        }

        let size = block_size as f32 * framebuffer.height as f32 / depth * scale;
        let center_x = (relative_angle / player.fov + 0.5) * framebuffer.width as f32;
        let left = center_x - size / 2.0;
        let top = (framebuffer.height as f32 - size) / 2.0;
        let start_x = left.max(0.0) as u32;
        let end_x = (left + size).min(framebuffer.width as f32) as u32;
        let start_y = top.max(0.0) as u32;
        let end_y = (top + size).min(framebuffer.height as f32) as u32;

        for x in start_x..end_x {
            if depth >= depth_buffer[x as usize] {
                continue;
            }
            let u = (x as f32 - left) / size;
            for y in start_y..end_y {
                let v = (y as f32 - top) / size;
                let color = texture_manager.get_sprite_pixel(marker, frame, u, v);
                if color.a > 0 {
                    framebuffer.set_current_color(color);
                    framebuffer.set_pixel(x, y);
                }
            }
        }
    }
}

fn distance_squared(a: Vector2, b: Vector2) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}
