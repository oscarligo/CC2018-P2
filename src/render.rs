use crate::caster::{cast_ray, Ray, RayIntersect};
use crate::framebuffer::Framebuffer;
use raylib::prelude::*;
use glam::Vec3A;
use crate::camera::Camera;
use crate::material::Light;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Flat,       // Base color only
    Normals,    // Normal vectors visualization
    BasicLight, // Basic lighting without shadows
    Full,       // Full lighting with shadows and reflections
}

#[inline(always)]
pub fn vec3_to_color(v: Vec3A) -> Color {
    Color::new(
        (v.x.clamp(0.0, 1.0) * 255.0) as u8,
        (v.y.clamp(0.0, 1.0) * 255.0) as u8,
        (v.z.clamp(0.0, 1.0) * 255.0) as u8,
        255,
    )
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[impl RayIntersect + Sync],
    lights: &[Light],
    camera: &Camera,
    fov_degrees: f32,
    mode: RenderMode,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov_scale = (fov_degrees.to_radians() * 0.5).tan();
    let ray_origin = camera.eye;


    
    framebuffer.render_parallel(|x, y| {
        // Convert pixel coordinates to normalized device coordinates (NDC)
        let screen_x = ((2.0 * (x as f32 + 0.5)) / width - 1.0) * aspect_ratio * fov_scale;
        let screen_y = (1.0 - (2.0 * (y as f32 + 0.5)) / height) * fov_scale;

        let local_dir = Vec3A::new(screen_x, screen_y, -1.0);
        let ray_direction = camera.basis_change(&local_dir).normalize();

        let ray = Ray {
            origin: ray_origin,
            direction: ray_direction,
        };

        let color = cast_ray(&ray, objects, lights, mode);
        vec3_to_color(color)
    });
}