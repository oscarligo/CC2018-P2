use crate::caster::{cast_ray, Ray, RayIntersect};
use crate::framebuffer::Framebuffer;
use glam::Vec3A;

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[impl RayIntersect + Sync],
    fov_degrees: f32,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    // Escala basada en el FOV vertical
    let fov_scale = (fov_degrees.to_radians() * 0.5).tan();
    let ray_origin = Vec3A::ZERO;

    // Paralelización por filas en CPU: cada hilo procesa un bloque de píxeles
    framebuffer.render_parallel(|x, y| {
        // Mapear al centro del píxel en coordenadas normalizadas [-1.0, 1.0]
        let screen_x = ((2.0 * (x as f32 + 0.5)) / width - 1.0) * aspect_ratio * fov_scale;
        let screen_y = (1.0 - (2.0 * (y as f32 + 0.5)) / height) * fov_scale;

        // glam ya provee .normalize() nativo altamente optimizado con SIMD
        let ray_direction = Vec3A::new(screen_x, screen_y, -1.0).normalize();
        let ray = Ray {
            origin: ray_origin,
            direction: ray_direction,
        };

        cast_ray(&ray, objects)
    });
}