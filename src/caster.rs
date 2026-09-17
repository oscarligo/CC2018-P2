use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;

pub struct Intersec{
    pub distance: f32,
    pub impact: char,
    pub hit_x: f32,
    pub hit_y: f32,
    pub vertical_hit: bool,
}


pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Maze,
    block_size: usize,
    angle_offset: f32,
    draw_line: bool,
    cast_step: f32
) -> Intersec {
    let mut d = 0.0;
    let ray_angle = player.angle + angle_offset;
    let ray_cos = ray_angle.cos();
    let ray_sin = ray_angle.sin();
    framebuffer.set_current_color(Color::WHITESMOKE);

    loop{
        let hit_x = player.position.x + d * ray_cos;
        let hit_y = player.position.y + d * ray_sin;
        let x = hit_x as usize;
        let y = hit_y as usize;

        let i = x / block_size;
        let j = y / block_size;

        if !matches!(maze[j][i], ' ' | 'p' | 'e' | 'w') {
            let previous_d = (d - cast_step).max(0.0);
            let previous_i = (player.position.x + previous_d * ray_cos) as usize / block_size;
            return Intersec {
                distance: d,
                impact: maze[j][i],
                hit_x,
                hit_y,
                vertical_hit: previous_i != i,
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        d += cast_step;
    }
    
}
