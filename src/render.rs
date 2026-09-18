use crate::caster::{cast_ray, Ray, RayIntersect};
use crate::framebuffer::Framebuffer;
use raylib::prelude::*;
use glam::Vec3A;
use crate::camera::Camera;
use crate::material::Light;
use crate::background::BackgroundTexture;
use crate::texture::Texture;

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
    background: &BackgroundTexture,
    textures: &[Texture],
    parallel_renderig: bool
) {

    if parallel_renderig {
        parallel_render(
            framebuffer,
            objects,
            lights,
            camera,
            fov_degrees,
            mode,
            background,
            textures
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
            textures
        );
    }

    
}



// Sequential rendering function (for testing purposes)
fn sequential_render(
    framebuffer: &mut Framebuffer,
    objects: &[impl RayIntersect + Sync],
    lights: &[Light],
    camera: &Camera,
    fov_degrees: f32,
    mode: RenderMode,
    background: &BackgroundTexture,
    textures: &[Texture]
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov_scale = (fov_degrees.to_radians() * 0.5).tan();
    let ray_origin = camera.eye;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = ((2.0 * (x as f32 + 0.5)) / width - 1.0) * aspect_ratio * fov_scale;
            let screen_y = (1.0 - (2.0 * (y as f32 + 0.5)) / height) * fov_scale;

            let local_dir = Vec3A::new(screen_x, screen_y, -1.0);
            let ray_direction = camera.basis_change(&local_dir).normalize();

            let ray = Ray {
                origin: ray_origin,
                direction: ray_direction,
            };

            let color = cast_ray(&ray, objects, lights, mode, background, textures);
            framebuffer.set_pixel_color(x, y, vec3_to_color(color));
        }
    }
}

// Parallel rendering function
fn parallel_render(
    framebuffer: &mut Framebuffer,
    objects: &[impl RayIntersect + Sync],
    lights: &[Light],
    camera: &Camera,
    fov_degrees: f32,
    mode: RenderMode,
    background: &BackgroundTexture,
    textures: &[Texture]
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov_scale = (fov_degrees.to_radians() * 0.5).tan();
    let ray_origin = camera.eye;

    framebuffer.render_parallel(|x, y| {
        let screen_x = ((2.0 * (x as f32 + 0.5)) / width - 1.0) * aspect_ratio * fov_scale;
        let screen_y = (1.0 - (2.0 * (y as f32 + 0.5)) / height) * fov_scale;

        let local_dir = Vec3A::new(screen_x, screen_y, -1.0);
        let ray_direction = camera.basis_change(&local_dir).normalize();

        let ray = Ray {
            origin: ray_origin,
            direction: ray_direction,
        };

        let color = cast_ray(&ray, objects, lights, mode, background, textures);
        vec3_to_color(color)
    });
}