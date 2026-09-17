mod framebuffer;
mod render;
mod caster;
mod sphere;
mod camera;
mod material;

use framebuffer::Framebuffer;
use render::render;
use sphere::Sphere;
use glam::Vec3A;
use raylib::prelude::*;
use camera::Camera;
use material::Material;



fn main() {
    let width = 1024;
    let height = 720;

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Raytracer")
        .build();

    let mut framebuffer = Framebuffer::new(&mut rl, &thread, width, height, Color::BLACK);

    let stone = Material {
        diffuse_color: Vec3A::new(0.8, 0.8, 0.8),
        albedo: [0.8, 0.2, 0.0, 0.0],
        specular_exponent: 100.0,
        refractive_index: 1.5,
    };

    let ruby = Material {
        diffuse_color: Vec3A::new(0.9, 0.1, 0.1),
        albedo: [0.6, 0.3, 0.1, 0.0],
        specular_exponent: 500.0,
        refractive_index: 1.5,
    };

    let objects: Vec<Sphere> = vec![
        Sphere {
            center: Vec3A::new(0.0, 0.0, -5.0),
            radius: 1.0,
            material: stone,
        },
        Sphere {
            center: Vec3A::new(4.0, 4.0, -6.0),
            radius: 1.0,
            material: stone,
        },
        Sphere {
            center: Vec3A::new(-1.0, 0.0, -4.0),
            radius: 1.0,
            material: ruby,
        },
        
    ];

    while !rl.window_should_close() {
        
        // Renderizar la escena
        render(&mut framebuffer, &objects, 70.0);

        // Mostrar el framebuffer en la ventana
        framebuffer.swap_buffers(&mut rl, &thread);
    }


}