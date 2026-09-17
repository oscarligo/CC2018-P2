mod framebuffer;
mod render;
mod caster;
mod sphere;

use framebuffer::Framebuffer;
use render::render;
use sphere::Sphere;
use glam::Vec3A;
use raylib::prelude::*;



fn main() {
    let width = 1024;
    let height = 720;

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Raytracer")
        .build();

    let mut framebuffer = Framebuffer::new(&mut rl, &thread, width, height, Color::BLACK);

    let objects: Vec<Sphere> = vec![
        Sphere {
            center: Vec3A::new(0.0, 0.0, -5.0),
            radius: 1.0,
            color: Color::RED,
        },
        Sphere {
            center: Vec3A::new(4.0, 4.0, -6.0),
            radius: 1.0,
            color: Color::GREEN,
        },
        Sphere {
            center: Vec3A::new(0.0, 0.0, -4.0),
            radius: 1.0,
            color: Color::BLUE,
        },
        
    ];

    while !rl.window_should_close() {
        
        // Renderizar la escena
        render(&mut framebuffer, &objects);

        // Mostrar el framebuffer en la ventana
        framebuffer.swap_buffers(&mut rl, &thread);
    }


}