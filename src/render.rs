use crate::caster::{cast_ray, Ray, RayIntersect};
use crate::framebuffer::Framebuffer;
use glam::Vec3A;
use crate::camera::Camera;

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[impl RayIntersect + Sync],
    camera: &Camera,
    fov_degrees: f32,
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

        cast_ray(&ray, objects)
    });
}