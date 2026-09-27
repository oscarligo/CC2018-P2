mod bvh;
mod camera;
mod caster;
mod cube;
mod events;
mod framebuffer;
mod material;
mod render;
mod world_gen;
mod textures;

use bvh::Bvh;
use camera::Camera;
use events::EventHandler;
use framebuffer::Framebuffer;
use glam::Vec3A;
use material::{Light, Material, MaterialTextureIds};
use raylib::prelude::*;
use render::{render, RenderMode};
use world_gen::terrain::WorldGenerator;
use textures::background::BackgroundTexture;
use textures::texture::Texture;

fn main() {
    let width = 1024;
    let height = 720;
    let parallel_rendering = true;

    // 
    const PREVIEW_PIXEL_SIZE: u32 = 2;
    const FULL_RENDER_PIXEL_SIZE: u32 = 1;
    const TERRAIN_GRID_SIZE: usize = 32;

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("CPU Ray Tracer")
        .build();

    let mut framebuffer = Framebuffer::new(&mut rl, &thread, width, height, Color::BLACK);

    let mut camera = Camera::new(
        Vec3A::new(0.0, 6.0, 12.0),
        Vec3A::new(0.0, 1.0, 0.0),
        Vec3A::new(0.0, 1.0, 0.0),
    );

    let textures = vec![
        Texture::load("assets/blocks/brick/diffuse.png", true),
        Texture::load("assets/blocks/brick/normal.png", false),
        Texture::load("assets/blocks/brick/specular.png", false),
        Texture::load("assets/blocks/stone/diffuse.png", true),
        Texture::load("assets/blocks/stone/normal.png", false),
        Texture::load("assets/blocks/stone/specular.png", false),
        Texture::load("assets/blocks/wool/diffuse.png", true),
        Texture::load("assets/blocks/wool/normal.png", false),
        Texture::load("assets/blocks/wool/specular.png", false),
        Texture::load("assets/blocks/glowstone/diffuse.png", true),
        Texture::load("assets/blocks/glowstone/normal.png", false),
        Texture::load("assets/blocks/glowstone/specular.png", false),
        Texture::load("assets/blocks/copper/diffuse.png", true),
        Texture::load("assets/blocks/copper/normal.png", false),
        Texture::load("assets/blocks/copper/specular.png", false),
        Texture::load("assets/blocks/gold_block/diffuse.png", true),
        Texture::load("assets/blocks/gold_block/normal.png", false),
        Texture::load("assets/blocks/gold_block/specular.png", false),
        Texture::load("assets/blocks/wood/diffuse.png", true),
        Texture::load("assets/blocks/wood/normal.png", false),
        Texture::load("assets/blocks/wood/specular.png", false),
        Texture::load("assets/blocks/white_glass/diffuse.png", true),
        Texture::load("assets/blocks/white_glass/normal.png", false),
        Texture::load("assets/blocks/white_glass/specular.png", false),
        Texture::load("assets/blocks/prismarine/diffuse.png", true),
        Texture::load("assets/blocks/prismarine/normal.png", false),
        Texture::load("assets/blocks/prismarine/specular.png", false),
        Texture::load("assets/blocks/ice/diffuse.png", true),
        Texture::load("assets/blocks/ice/normal.png", false),
        Texture::load("assets/blocks/ice/specular.png", false),
        Texture::load("assets/blocks/grass/diffuse.png", true),
        Texture::load("assets/blocks/grass/normal.png", false),
        Texture::load("assets/blocks/grass/specular.png", false),
        Texture::load("assets/blocks/moss/diffuse.png", true),
        Texture::load("assets/blocks/moss/normal.png", false),
        Texture::load("assets/blocks/moss/specular.png", false),
        Texture::load("assets/blocks/lantern/diffuse.png", true),
        Texture::load("assets/blocks/lantern/normal.png", false),
        Texture::load("assets/blocks/lantern/specular.png", false),
        Texture::load("assets/blocks/q_brick/diffuse.png", true),
        Texture::load("assets/blocks/q_brick/normal.png", false),
        Texture::load("assets/blocks/q_brick/specular.png", false),
        Texture::load("assets/blocks/leaves/diffuse.png", true),
        Texture::load("assets/blocks/leaves/normal.png", false),
        Texture::load("assets/blocks/leaves/specular.png", false),
        Texture::load("assets/blocks/cobblestone/diffuse.png", true),
        Texture::load("assets/blocks/cobblestone/normal.png", false),
        Texture::load("assets/blocks/cobblestone/specular.png", false),
        Texture::load("assets/blocks/log/diffuse.png", true),
        Texture::load("assets/blocks/log/normal.png", false),
        Texture::load("assets/blocks/log/specular.png", false),
    ];

    let background_texture = BackgroundTexture::load("assets/sky.png");
    let mut render_mode = RenderMode::Flat;

    let brick = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
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
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(3),
            normal_id: Some(4),
            specular_id: Some(5),
        },
    };

    let grass = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(30),
            normal_id: Some(31),
            specular_id: Some(32),
        },
    };


    let wool = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [1.0, 0.0, 0.0, 0.0],
        specular_exponent: 1.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(6),
            normal_id: Some(7),
            specular_id: Some(8),
        },
    };

    let glowstone = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [1.0, 0.0, 0.0, 0.0],
        specular_exponent: 1.0,
        refractive_index: 1.0,
        emission_strength: 5.5,
        emission_color: Vec3A::new(1.0, 0.9, 0.7),
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
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
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
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
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
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
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
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(21),
            normal_id: Some(22),
            specular_id: Some(23),
        },
    };

    let no_texture_glass: Material = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.0, 0.5, 0.1, 0.8],
        specular_exponent: 125.0,
        refractive_index: 1.52,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds::empty(),
    };

    let prismarine = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.35, 0.55, 0.15, 0.0],
        specular_exponent: 90.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
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
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(27),
            normal_id: Some(28),
            specular_id: Some(29),
        },
    };

    let grass = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(30),
            normal_id: Some(31),
            specular_id: Some(32),
        },
    };

    let moss = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(33),
            normal_id: Some(34),
            specular_id: Some(35),
        },
    };

    let lantern = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 10.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(36),
            normal_id: Some(37),
            specular_id: Some(38),
        },
    };

    let q_brick = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(39),
            normal_id: Some(40),
            specular_id: Some(41),
        },
    };

    let leaves = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(42),
            normal_id: Some(43),
            specular_id: Some(44),
        },
    };

    let cobblestone = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(45),
            normal_id: Some(46),
            specular_id: Some(47),
        },
    };

    let log = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(48),
            normal_id: Some(49),
            specular_id: Some(50),
        },
    };


    let sun = Light {
        position: Vec3A::new(0.0, 20.0, 50.0),
        intensity: 3.0,
        color: Vec3A::ONE,
        attenuation: 0.0,
    };

    let mut lights = vec![sun];
    
    let mut generator = WorldGenerator::new(TERRAIN_GRID_SIZE, 16, TERRAIN_GRID_SIZE, 1.0);
    let cubes = generator.generate(stone,ice,brick, wood, gold_block, white_glass, cobblestone, q_brick, copper, log, log, moss, glowstone);

    let mut scene = Bvh::new(cubes);
    lights.extend(scene.emissive_lights());
    let event_handler = EventHandler::default();
    let mut needs_full_render = true; 

    while !rl.window_should_close() {
        let (changed, is_moving, (dx, dz)) =
            event_handler.handle_events(&rl, &mut camera, &mut render_mode);

        if dx != 0 || dz != 0 {
            generator.shift(dx, dz);
            let new_cubes = generator.generate( stone, ice, brick, wood, gold_block, white_glass, cobblestone,q_brick, copper, log, log, moss, glowstone);
            scene = Bvh::new(new_cubes);
            lights.clear();
            lights.push(sun);
            lights.extend(scene.emissive_lights());
        }

        let rendered = changed || needs_full_render;

        if rendered {
            let current_pixel_size = if is_moving {
                PREVIEW_PIXEL_SIZE
            } else {
                FULL_RENDER_PIXEL_SIZE
            };

            render(
                &mut framebuffer,
                &scene,
                &lights,
                &camera,
                60.0,
                render_mode,
                &background_texture,
                &textures,
                parallel_rendering,
                current_pixel_size,
            );

            needs_full_render = is_moving;
        }

        framebuffer.swap_buffers(&mut rl, &thread, rendered);
    }
}
