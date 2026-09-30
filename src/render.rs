use crate::camera::Camera;
use crate::bvh::Bvh;
use crate::caster::{cast_ray, Ray};
use crate::framebuffer::Framebuffer;
use crate::material::Light;
use crate::textures::background::BackgroundTexture;
use crate::textures::texture::Texture;
use crate::vector::Vec3A;
use raylib::prelude::*;

/*
    This module contains the rendering logic for the 3D scene.
    It handles the different rendering modes and the conversion of 3D vectors to colors.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Flat,       // Base color only
    Normals,    // Normal vectors visualization
    BasicLight, // Basic lighting without shadows
    Full,       // Full lighting with shadows and reflections
}

#[inline(always)]
pub fn vec3_to_color(v: Vec3A) -> Color {
    let clamped = v.clamp(Vec3A::ZERO, Vec3A::splat(1.0)) * 255.0;
    Color::new(clamped.x as u8, clamped.y as u8, clamped.z as u8, 255)
}

#[derive(Clone, Copy)]
struct ScreenProjection {
    scale_x: f32,
    offset_x: f32,
    scale_y: f32,
    offset_y: f32,
}

impl ScreenProjection {
    #[inline(always)]
    fn new(width: f32, height: f32, fov_degrees: f32) -> Self {
        let aspect_ratio = width / height;
        let fov_scale = (fov_degrees.to_radians() * 0.5).tan();

        let inv_width = 1.0 / width;
        let inv_height = 1.0 / height;

        Self {
            scale_x: 2.0 * inv_width * aspect_ratio * fov_scale,
            offset_x: aspect_ratio * fov_scale,
            scale_y: 2.0 * inv_height * fov_scale,
            offset_y: fov_scale,
        }
    }

    #[inline(always)]
    fn screen_x(&self, x: f32) -> f32 {
        (x + 0.5) * self.scale_x - self.offset_x
    }

    #[inline(always)]
    fn screen_y(&self, y: f32) -> f32 {
        self.offset_y - (y + 0.5) * self.scale_y
    }
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &Bvh,
    lights: &[Light],
    camera: &Camera,
    fov_degrees: f32,
    mode: RenderMode,
    background: &BackgroundTexture,
    textures: &[Texture],
    parallel_rendering: bool,
    pixel_size: u32,
) {
    if parallel_rendering {
        parallel_render(
            framebuffer,
            objects,
            lights,
            camera,
            fov_degrees,
            mode,
            background,
            textures,
            pixel_size,
        );
    } else {
        sequential_render(
            framebuffer,
            objects,
            lights,
            camera,
            fov_degrees,
            mode,
            background,
            textures,
            pixel_size,
        );
    }
}

// Sequential rendering function (for testing purposes)
fn sequential_render(
    framebuffer: &mut Framebuffer,
    objects: &Bvh,
    lights: &[Light],
    camera: &Camera,
    fov_degrees: f32,
    mode: RenderMode,
    background: &BackgroundTexture,
    textures: &[Texture],
    pixel_size: u32,
) {
    let proj = ScreenProjection::new(
        framebuffer.width as f32,
        framebuffer.height as f32,
        fov_degrees,
    );
    let ray_origin = camera.eye;

    let pixel_size = pixel_size.max(1);
    for y in (0..framebuffer.height).step_by(pixel_size as usize) {
        let sample_y = (y + pixel_size / 2).min(framebuffer.height - 1);
        let screen_y = proj.screen_y(sample_y as f32);

        for x in (0..framebuffer.width).step_by(pixel_size as usize) {
            let sample_x = (x + pixel_size / 2).min(framebuffer.width - 1);
            let screen_x = proj.screen_x(sample_x as f32);

            let local_dir = Vec3A::new(screen_x, screen_y, -1.0);
            let ray_direction = camera.basis_change(local_dir).normalize();

            let ray = Ray::new(ray_origin, ray_direction);
            let color = cast_ray(&ray, objects, lights, mode, background, textures);
            framebuffer.set_pixel_block(x, y, pixel_size, vec3_to_color(color));
        }
    }
}

// Parallel rendering function
fn parallel_render(
    framebuffer: &mut Framebuffer,
    objects: &Bvh,
    lights: &[Light],
    camera: &Camera,
    fov_degrees: f32,
    mode: RenderMode,
    background: &BackgroundTexture,
    textures: &[Texture],
    pixel_size: u32,
) {
    let proj = ScreenProjection::new(
        framebuffer.width as f32,
        framebuffer.height as f32,
        fov_degrees,
    );
    let ray_origin = camera.eye;

    framebuffer.render_parallel(pixel_size, |x, y| {
        let screen_x = proj.screen_x(x as f32);
        let screen_y = proj.screen_y(y as f32);

        let local_dir = Vec3A::new(screen_x, screen_y, -1.0);
        let ray_direction = camera.basis_change(local_dir).normalize();

        let ray = Ray::new(ray_origin, ray_direction);
        let color = cast_ray(&ray, objects, lights, mode, background, textures);
        vec3_to_color(color)
    });
}
