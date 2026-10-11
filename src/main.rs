mod bvh;
mod camera;
mod caster;
mod cube;
mod events;
mod framebuffer;
mod material;
mod render;
mod textures;
mod triangle;
mod vector;
mod world_gen;

use bvh::Bvh;
use camera::Camera;
use events::EventHandler;
use framebuffer::Framebuffer;
use material::{Light, Material, MaterialTextureIds};
use raylib::prelude::*;
use render::{RenderMode, render};
use textures::background::BackgroundTexture;
use textures::texture::Texture;
use triangle::load_obj;
use vector::Vec3A;
use world_gen::terrain::WorldGenerator;

const JUMP_SPEED: f32 = 7.5;
const GRAVITY: f32 = 24.0;

fn update_jump(
    character_y: &mut f32,
    velocity: &mut f32,
    ground_y: f32,
    jump: bool,
    delta_time: f32,
) -> bool {
    if jump && (*character_y - ground_y).abs() <= 0.001 && *velocity == 0.0 {
        *velocity = JUMP_SPEED;
    }
    if *velocity == 0.0 && *character_y <= ground_y {
        *character_y = ground_y;
        return false;
    }

    let previous_y = *character_y;
    *velocity -= GRAVITY * delta_time;
    *character_y += *velocity * delta_time;
    if *character_y <= ground_y {
        *character_y = ground_y;
        *velocity = 0.0;
    }
    *character_y != previous_y
}

fn main() {
    let width = 1024;
    let height = 720;
    let parallel_rendering = true;

    const PREVIEW_PIXEL_SIZE: u32 = 2;
    const FULL_RENDER_PIXEL_SIZE: u32 = 1;
    const TERRAIN_GRID_SIZE: usize = 32;
    const CHARACTER_HEIGHT: f32 = 2.0;

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

    let mut textures = vec![
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
        Texture::load("assets/blocks/dark_oak/diffuse.png", true),
        Texture::load("assets/blocks/dark_oak/normal.png", false),
        Texture::load("assets/blocks/dark_oak/specular.png", false),
        Texture::load("assets/blocks/bamboo/diffuse.png", true),
        Texture::load("assets/blocks/bamboo/normal.png", false),
        Texture::load("assets/blocks/bamboo/specular.png", false),
        Texture::load("assets/blocks/sand/diffuse.png", true),
        Texture::load("assets/blocks/sand/normal.png", false),
        Texture::load("assets/blocks/sand/specular.png", false),
    ];
    let character_texture_id = textures.len();
    textures.push(Texture::load_resized("assets/character.jpg", true, 1024));

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

    let bamboo = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(54),
            normal_id: Some(55),
            specular_id: Some(56),
        },
    };

    let sand = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(57),
            normal_id: Some(58),
            specular_id: Some(59),
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

    let dark_oak_wood = Material {
        diffuse_color: Vec3A::splat(1.0),
        albedo: [0.35, 0.55, 0.15, 0.0],
        specular_exponent: 90.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(51),
            normal_id: Some(52),
            specular_id: Some(53),
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

    let character_material = Material {
        diffuse_color: Vec3A::ONE,
        albedo: [0.95, 0.05, 0.0, 0.0],
        specular_exponent: 6.0,
        refractive_index: 1.0,
        emission_strength: 0.0,
        emission_color: Vec3A::ONE,
        textures: MaterialTextureIds {
            diffuse_id: Some(character_texture_id),
            normal_id: None,
            specular_id: None,
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
    let character = load_obj("assets/character.obj", CHARACTER_HEIGHT, character_material)
        .unwrap_or_else(|error| panic!("No se pudo cargar el personaje: {error}"));
    let mut cubes = generator.generate(
        stone,
        ice,
        brick,
        wood,
        gold_block,
        white_glass,
        cobblestone,
        prismarine,
        copper,
        log,
        log,
        moss,
        glowstone,
    );
    let mut ground_y = generator.center_ground_height();
    let mut character_y = ground_y;
    let character_at_ground = character
        .iter()
        .map(|triangle| triangle.translated(Vec3A::new(0.0, character_y, 0.0)))
        .collect();

    let mut scene = Bvh::new(cubes.clone(), character_at_ground);
    lights.extend(scene.emissive_lights());
    let event_handler = EventHandler::default();
    let mut needs_full_render = true;
    let mut character_angle = 0.0_f32;
    let mut vertical_velocity = 0.0;

    while !rl.window_should_close() {
        let (changed, is_moving, (dx, dz), jump_pressed) =
            event_handler.handle_events(&rl, &mut camera, &mut render_mode);
        let movement_requested = dx != 0 || dz != 0;

        if movement_requested {
            character_angle = (-dz as f32).atan2(dx as f32);
            if let Some(target_ground) =
                generator.movement_ground(dx, dz, character_y, CHARACTER_HEIGHT)
            {
                generator.shift(dx, dz);
                ground_y = target_ground;
                cubes = generator.generate(
                    stone,
                    ice,
                    brick,
                    wood,
                    gold_block,
                    white_glass,
                    cobblestone,
                    prismarine,
                    copper,
                    log,
                    log,
                    moss,
                    glowstone,
                );
            }
        }

        let previous_y = character_y;
        let mut character_moved = update_jump(
            &mut character_y,
            &mut vertical_velocity,
            ground_y,
            jump_pressed,
            rl.get_frame_time().min(0.05),
        );
        if character_y > previous_y && generator.player_collides(character_y, CHARACTER_HEIGHT) {
            character_y = previous_y;
            vertical_velocity = 0.0;
            character_moved = true;
        }

        if movement_requested || character_moved {
            let character_at_ground = character
                .iter()
                .map(|triangle| {
                    triangle.rotated_y(character_angle).translated(Vec3A::new(
                        0.0,
                        character_y,
                        0.0,
                    ))
                })
                .collect();
            scene = Bvh::new(cubes.clone(), character_at_ground);
            lights.clear();
            lights.push(sun);
            lights.extend(scene.emissive_lights());
        }

        let rendered = changed || character_moved || needs_full_render;

        if rendered {
            let current_pixel_size = if is_moving || character_moved {
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

            needs_full_render = is_moving || character_moved;
        }

        framebuffer.swap_buffers(&mut rl, &thread, rendered);
    }
}
