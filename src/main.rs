mod camera;
mod caster;
mod cube;
mod events;
mod framebuffer;
mod material;
mod render;
mod sphere;

use camera::Camera;
use cube::Cube;
use events::EventHandler;
use framebuffer::Framebuffer;
use glam::Vec3A;
use material::{Light, Material};
use raylib::prelude::*;
use caster::RayIntersect;
use render::{render, RenderMode};

use crate::sphere::Sphere;

fn main() {
    let width = 1024;
    let height = 720;

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Raytracer en CPU")
        .build();

    let mut framebuffer = Framebuffer::new(&mut rl, &thread, width, height, Color::BLACK);

    let mut camera = Camera::new(
        Vec3A::new(0.0, 0.0, 5.0),
        Vec3A::new(0.0, 0.0, -5.0),
        Vec3A::new(0.0, 1.0, 0.0),
    );

    let mut render_mode = RenderMode::Full;

    let stone = Material {
        diffuse_color: Vec3A::new(0.65, 0.65, 0.68),
        albedo: [0.85, 0.15, 0.0, 0.0],
        specular_exponent: 25.0,
        refractive_index: 1.0,
    };

    let ruby = Material {
        diffuse_color: Vec3A::new(0.95, 0.05, 0.15),
        albedo: [0.5, 0.4, 0.1, 0.0],
        specular_exponent: 350.0,
        refractive_index: 1.77,
    };

    let gold = Material {
        diffuse_color: Vec3A::new(1.0, 0.84, 0.0),
        albedo: [0.2, 0.6, 0.7, 0.0],
        specular_exponent: 600.0,
        refractive_index: 1.0,
    };

    let emerald = Material {
        diffuse_color: Vec3A::new(0.08, 0.75, 0.28),
        albedo: [0.55, 0.35, 0.1, 0.0],
        specular_exponent: 250.0,
        refractive_index: 1.58,
    };

    let copper = Material {
        diffuse_color: Vec3A::new(0.85, 0.45, 0.25),
        albedo: [0.35, 0.45, 0.4, 0.0],
        specular_exponent: 200.0,
        refractive_index: 1.0,
    };

    let chrome = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.0, 10.0, 0.95, 0.0],
        specular_exponent: 1425.0,
        refractive_index: 1.0,
    };

    let glass = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.0, 0.5, 0.1, 0.8],
        specular_exponent: 125.0,
        refractive_index: 1.52,
    };

    let ground = Material {
        diffuse_color: Vec3A::new(0.4, 0.4, 0.4),
        albedo: [0.9, 0.1, 0.0, 0.0],
        specular_exponent: 50.0,
        refractive_index: 1.0,
    };

    let objects: Vec<Sphere> = vec![
        Sphere {
            center: Vec3A::new(0.0, -1001.0, -6.0),
            radius: 1000.0,
            material: ground,
        },
        Sphere {
            center: Vec3A::new(0.0, 0.0, -6.0),
            radius: 1.0,
            material: chrome,
        },

        Sphere {
            center: Vec3A::new(-2.2, -0.3, -4.8),
            radius: 0.7,
            material: glass,
        },

        Sphere {
            center: Vec3A::new(1.9, -0.2, -4.5),
            radius: 0.8,
            material: ruby,
        },

        Sphere {
            center: Vec3A::new(-3.6, 1.2, -8.0),
            radius: 1.1,
            material: gold,
        },

        Sphere {
            center: Vec3A::new(-0.8, -0.6, -3.2),
            radius: 0.4,
            material: emerald,
        },

        Sphere {
            center: Vec3A::new(3.8, 1.0, -8.5),
            radius: 1.3,
            material: stone,
        },

        Sphere {
            center: Vec3A::new(0.8, 1.6, -7.5),
            radius: 0.6,
            material: copper,
        },

        Sphere {
            center: Vec3A::new(1.1, -0.65, -3.0),
            radius: 0.35,
            material: glass,
        },
    ];

    let lights = vec![
        Light {
            position: Vec3A::new(-20.0, 20.0, 20.0),
            intensity: 3.0,
            color: Vec3A::new(1.0, 1.0, 1.0),
        },
        Light {
            position: Vec3A::new(10.0, 5.0, 0.0),
            intensity: 1.0,
            color: Vec3A::new(0.8, 0.85, 1.0), // Luz de relleno suave
        },
    ];

    let event_handler = EventHandler::new(2.0, 0.5, 0.005);

    render(&mut framebuffer, &objects, &lights, &camera, 60.0, render_mode);

    while !rl.window_should_close() {
        let changed = event_handler.handle_events(&rl, &mut camera, &mut render_mode);

        if changed {
            render(&mut framebuffer, &objects, &lights, &camera, 60.0, render_mode);
        }

        framebuffer.swap_buffers(&mut rl, &thread);
    }
}