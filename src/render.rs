use crate::framebuffer::Framebuffer;
use crate::caster::{cast_ray, Ray};
use glam::Vec3A;
use crate::caster::RayIntersect;



pub fn render(framebuffer: &mut Framebuffer, objects: &[impl RayIntersect]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3A::new(screen_x, screen_y, -1.0));
            let ray_origin = Vec3A::new(0.0, 0.0, 0.0);
            let ray = Ray { origin: ray_origin, direction: ray_direction };

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&ray, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_pixel_color(x, y, pixel_color);
    
        }
    }
}


fn normalize(v: &Vec3A) -> Vec3A {
    v.normalize()
}  