mod background;
mod camera;
mod caster;
mod cube;
mod events;
mod framebuffer;
mod material;
mod render;
mod texture;

use background::BackgroundTexture;
use camera::Camera;
use cube::Cube;
use events::EventHandler;
use framebuffer::Framebuffer;
use glam::Vec3A;
use material::{Light, Material, MaterialTextureIds};
use raylib::prelude::*;
use render::{render, RenderMode};
use texture::Texture;

fn main() {
    let width = 1024;
    let height = 720;

    let paralel_rendering = true;

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("CPU Ray Tracer")
        .build();

    // Framebuffer initialization.
    let mut framebuffer = Framebuffer::new(&mut rl, &thread, width, height, Color::BLACK);

    // Camera initialization.
    let mut camera = Camera::new(
        Vec3A::new(0.0, 0.0, 5.0),
        Vec3A::new(0.0, 0.0, -5.0),
        Vec3A::new(0.0, 1.0, 0.0),
    );

    // Load textures for various materials used in the scene. 
    // sEach texture is loaded from a file and can be either sRGB or linear.
    let textures = vec![
        Texture::load("assets/blocks/brick/diffuse.png", true),// ID 0 (sRGB)
        Texture::load("assets/blocks/brick/normal.png", false), 
        Texture::load("assets/blocks/brick/specular.png", false),
        Texture::load("assets/blocks/stone/diffuse.png", true),// ID 3 (Lineal)
        Texture::load("assets/blocks/stone/normal.png", false),
        Texture::load("assets/blocks/stone/specular.png", false),
        Texture::load("assets/blocks/wool/diffuse.png", true),// ID 6 (Lineal)
        Texture::load("assets/blocks/wool/normal.png", false),
        Texture::load("assets/blocks/wool/specular.png", false),
        Texture::load("assets/blocks/glowstone/diffuse.png", true),// ID 9 (Lineal)
        Texture::load("assets/blocks/glowstone/normal.png", false),
        Texture::load("assets/blocks/glowstone/specular.png", false),
        Texture::load("assets/blocks/copper/diffuse.png", true),// ID 12 (Lineal)
        Texture::load("assets/blocks/copper/normal.png", false),
        Texture::load("assets/blocks/copper/specular.png", false),
        Texture::load("assets/blocks/gold_block/diffuse.png", true),// ID 15 (Lineal)
        Texture::load("assets/blocks/gold_block/normal.png", false),
        Texture::load("assets/blocks/gold_block/specular.png", false),
        Texture::load("assets/blocks/wood/diffuse.png", true),// ID 18 (Lineal)
        Texture::load("assets/blocks/wood/normal.png", false),
        Texture::load("assets/blocks/wood/specular.png", false),
        Texture::load("assets/blocks/white_glass/diffuse.png", true),// ID 21 (Lineal)
        Texture::load("assets/blocks/white_glass/normal.png", false),
        Texture::load("assets/blocks/white_glass/specular.png", false),
        Texture::load("assets/blocks/prismarine/diffuse.png", true),// ID 24 (Lineal)
        Texture::load("assets/blocks/prismarine/normal.png", false),
        Texture::load("assets/blocks/prismarine/specular.png", false),
        Texture::load("assets/blocks/ice/diffuse.png", true),// ID 27 (Lineal)
        Texture::load("assets/blocks/ice/normal.png", false),
        Texture::load("assets/blocks/ice/specular.png", false),
        
    ];

    // Load the background texture for the scene, which will be used to render 
    // the sky or environment behind the objects.
    let background_texture = BackgroundTexture::load("assets/sky.png");

    // Initial render mode is set to Full, which means all rendering features are enabled.
    let mut render_mode = RenderMode::BasicLight;

    // The following materials are defined with their respective properties and associated textures.
    // Each material has a diffuse color, albedo values, specular exponent, refractive index, 
    // and texture IDs for diffuse, normal, and specular maps.
    let brick = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(0),
            normal_id: Some(1),
            specular_id: Some(2),
            },
    };


    let stone = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 8.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(3),
            normal_id: Some(4),
            specular_id: Some(5),
        },
    };

    let wool = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [1.0, 0.0, 0.0, 0.0],
        specular_exponent: 1.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(6),
            normal_id: Some(7),
            specular_id: Some(8),
        },
    };

    let glowstone = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [1.5, 0.0, 0.0, 0.0],  
        specular_exponent: 1.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(9),
            normal_id: Some(10),
            specular_id: Some(11),
        },
    };

    let copper = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.35, 0.55, 0.15, 0.0], 
        specular_exponent: 90.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(12),
            normal_id: Some(13),
            specular_id: Some(14),
        },
    };

    let gold_block = Material {
        diffuse_color: Vec3A::splat(1.0), 
        albedo: [0.35, 0.55, 0.15, 0.0],  
        specular_exponent: 90.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(15),
            normal_id: Some(16),
            specular_id: Some(17),
        },
    };

    let wood = Material {
        diffuse_color: Vec3A::splat(1.0), 
        albedo: [0.35, 0.55, 0.15, 0.0], 
        specular_exponent: 90.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(18),
            normal_id: Some(19),
            specular_id: Some(20),
        },
    };

    let white_glass = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.0, 0.5, 0.1, 0.8],    
        specular_exponent: 125.0,
        refractive_index: 1.52,
        textures: MaterialTextureIds {
            diffuse_id: Some(21),
            normal_id: Some(22),
            specular_id: Some(23),
        },
    };

    let prismarine = Material {
        diffuse_color: Vec3A::splat(1.0), 
        albedo: [0.35, 0.55, 0.15, 0.0],  
        specular_exponent: 90.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(24),
            normal_id: Some(25),
            specular_id: Some(26),
        },
    };  

    let ice = Material {
        diffuse_color: Vec3A::splat(1.0), 
        albedo: [0.0, 10.0, 0.95, 0.0], 
        specular_exponent: 1425.0,
        refractive_index: 1.0,
        textures: MaterialTextureIds {
            diffuse_id: Some(27),
            normal_id: Some(28),
            specular_id: Some(29),
        },
    };

    let objects: Vec<Cube> = vec![
        Cube {
            center: Vec3A::new(0.0, 0.0, -6.0),
            size: 1.3,
            material: ice,
        },
        Cube {
            center: Vec3A::new(-2.2, -0.3, -4.8),
            size: 1.4,
            material: white_glass,
        },
        Cube {
            center: Vec3A::new(1.9, -0.2, -4.5),
            size: 1.6,
            material: wood,
        },
        Cube {
            center: Vec3A::new(-3.6, 1.2, -8.0),
            size: 2.2,
            material: stone,
        },
        Cube {
            center: Vec3A::new(-0.8, -1.0, -3.2),
            size: 1.0,
            material: wool,
        },
        Cube {
            center: Vec3A::new(3.8, 1.0, -8.5),
            size: 2.6,
            material: brick,
        },
        Cube {
            center: Vec3A::new(0.8, 1.6, -8.5),
            size: 2.5,
            material: copper,
        },
        Cube {
            center: Vec3A::new(1.1, -0.65, -3.0),
            size: 0.7,
            material: prismarine,
        },

        Cube {
            center: Vec3A::new(-1.5, 1.8, -6.5),
            size: 1.0,
            material:   gold_block,
        },
        Cube {
            center: Vec3A::new(3.5, -1.0, -5.0),
            size: 1.0,
            material:   glowstone,
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
            color: Vec3A::new(0.8, 0.85, 1.0),
        },
    ];
    

    let event_handler = EventHandler::default();

    // Initial render of the scene with the specified 
    // framebuffer, objects, lights, camera, render mode, background texture, and textures.
    render(
        &mut framebuffer,
        &objects,
        &lights,
        &camera,
        60.0,
        render_mode,
        &background_texture,
        &textures,
        paralel_rendering
    );

    while !rl.window_should_close() {
        let changed = event_handler.handle_events(&rl, &mut camera, &mut render_mode);

        if changed {
            render(
                &mut framebuffer,
                &objects,
                &lights,
                &camera,
                60.0,
                render_mode,
                &background_texture,
                &textures,
                paralel_rendering
            );
        }

        framebuffer.swap_buffers(&mut rl, &thread);
    }
}