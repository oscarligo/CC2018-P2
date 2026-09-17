mod framebuffer;
mod render;
mod maze;
mod player;
mod caster;
mod events;
mod texture_manager;
mod enemy;
mod screens;
mod portal;
mod projectile;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use player::Player;
use std::f32::consts::PI;
use std::{thread};
use std::time::Duration;
use maze::*;
use caster::cast_ray;  
use render::*;
use events::process_events;
use texture_manager::TextureManager;
use enemy::Enemy;
use screens::*;
use portal::Portal;
use projectile::Projectile;

#[derive(Clone, Copy)]
enum Screen {
    Main,
    Worlds(usize),
    Game,
    Death,
}

const WORLD_MAPS: [&str; 2] = ["src/world_1.txt", "src/world_2.txt"];

fn main() {
    let window_width: i32 = 1080;
    let window_height: i32 = 600;
    
    let framebuffer_width = window_width as u32;
    let framebuffer_height: u32 = window_height as u32;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Example")
        .build();

    let audio = RaylibAudio::init_audio_device().expect("Failed to initialize audio");
    let arrow_sound = audio.new_sound("assets/audio/arrow.mp3")
        .expect("Failed to load arrow sound");
    let click_sound = audio.new_sound("assets/audio/click.mp3")
        .expect("Failed to load menu click sound");
    let world_music = [
        audio.new_music("assets/audio/world_1.mp3")
            .expect("Failed to load world 1 music"),
        audio.new_music("assets/audio/world_2.mp3")
            .expect("Failed to load world 2 music"),
    ];

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);

    let mut maze: Maze = load_maze(WORLD_MAPS[0]);

    let block_size = 10;
    let player_movement_speed = block_size as f32 / 5.0;
    let player_rotation_speed = block_size as f32 / 50.0;
    let mut player = Player::new(Vector2::new(0.0, 0.0), PI/4.0, player_movement_speed, player_rotation_speed, PI/3.0);
    player.set_initial_position(&maze, block_size);
    let num_rays = 5; 

    let texture_manager = TextureManager::new();
    let mut enemies = Enemy::from_maze(&maze, block_size);
    let mut portals = Portal::from_maze(&maze, block_size);
    let mut projectiles = Vec::new();
    let mut current_world = 0;
    let main_screen = window.load_texture(&thread, "assets/main_screen.png")
        .expect("Failed to load main screen");
    let death_screen = window.load_texture(&thread, "assets/death_screen.png")
        .expect("Failed to load death screen");
    let world_screens = [
        window.load_texture(&thread, "assets/world_1.png")
            .expect("Failed to load world 1 screen"),
        window.load_texture(&thread, "assets/world_2.png")
            .expect("Failed to load world 2 screen"),
    ];
    let mut screen = Screen::Main;



    while !window.window_should_close() {
        if !matches!(screen, Screen::Game)
            && [KeyboardKey::KEY_ENTER, KeyboardKey::KEY_W, KeyboardKey::KEY_S]
                .iter()
                .any(|&key| window.is_key_pressed(key))
        {
            click_sound.play();
        }

        match screen {
            Screen::Main => {
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    screen = Screen::Worlds(0);
                }
                draw_screen(&mut window, &thread, &main_screen, window_width, window_height);
            }
            Screen::Worlds(mut selected) => {
                if window.is_key_pressed(KeyboardKey::KEY_W)
                    || window.is_key_pressed(KeyboardKey::KEY_S)
                {
                    selected = next_world(selected);
                }

                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    current_world = selected;
                    maze = load_maze(WORLD_MAPS[selected]);
                    player.set_initial_position(&maze, block_size);
                    enemies = Enemy::from_maze(&maze, block_size);
                    portals = Portal::from_maze(&maze, block_size);
                    projectiles.clear();
                    play_world_music(&world_music, current_world);
                    window.disable_cursor();
                    screen = Screen::Game;
                } else {
                    screen = Screen::Worlds(selected);
                }

                draw_screen(
                    &mut window,
                    &thread,
                    &world_screens[selected],
                    window_width,
                    window_height,
                );
            }
            Screen::Game => {
                framebuffer.clear();
                process_events(&mut window, &mut player, &maze, block_size);
                if player_touches_enemy(&player, &enemies, block_size) {
                    world_music[current_world].stop_stream();
                    window.enable_cursor();
                    screen = Screen::Death;
                    continue;
                }
                if player_on_marker(&player, &maze, block_size, 'w') {
                    current_world = next_world(current_world);
                    maze = load_maze(WORLD_MAPS[current_world]);
                    player.set_initial_position(&maze, block_size);
                    enemies = Enemy::from_maze(&maze, block_size);
                    portals = Portal::from_maze(&maze, block_size);
                    projectiles.clear();
                    play_world_music(&world_music, current_world);
                }
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    projectiles.push(Projectile::new(player.position, player.angle, block_size));
                    arrow_sound.play();
                }

                let delta_time = window.get_frame_time();
                world_music[current_world].update_stream();
                enemies.iter_mut().for_each(|enemy| enemy.update(delta_time));
                portals.iter_mut().for_each(|portal| portal.update(delta_time));
                projectiles.retain_mut(|projectile| {
                    projectile.update(delta_time, &maze, block_size)
                });
                let depth_buffer = render::render3d(
                    &mut framebuffer,
                    &player,
                    &maze,
                    block_size,
                    current_world,
                    &texture_manager,
                );
                render::render_sprites(
                    &mut framebuffer,
                    &player,
                    &enemies,
                    &portals,
                    &projectiles,
                    block_size,
                    &depth_buffer,
                    &texture_manager,
                );

                render_maze(&mut framebuffer, &maze, block_size);
                render_player(&mut framebuffer, &mut player);

                for i in 0..num_rays {
                    let current_ray = i as f32 / num_rays as f32;
                    let angle_offset = (current_ray - 0.5) * player.fov;
                    cast_ray(
                        &mut framebuffer,
                        &player,
                        &maze,
                        block_size,
                        angle_offset,
                        true,
                        8.0,
                    );
                }

                framebuffer.swap_buffers(&mut window, &thread);
            }
            Screen::Death => {
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    screen = Screen::Main;
                }
                draw_screen(&mut window, &thread, &death_screen, window_width, window_height);
            }
        }
        thread::sleep(Duration::from_millis(16));
    }
}

fn player_touches_enemy(player: &Player, enemies: &[Enemy], block_size: usize) -> bool {
    let contact_distance = block_size as f32 * 0.7;
    enemies.iter().any(|enemy| {
        let dx = player.position.x - enemy.position.x;
        let dy = player.position.y - enemy.position.y;
        dx * dx + dy * dy <= contact_distance * contact_distance
    })
}

fn play_world_music(music: &[Music<'_>; 2], world: usize) {
    music.iter().for_each(Music::stop_stream);
    music[world].play_stream();
}

fn player_on_marker(player: &Player, maze: &Maze, block_size: usize, marker: char) -> bool {
    maze.get(player.position.y as usize / block_size)
        .and_then(|row| row.get(player.position.x as usize / block_size))
        == Some(&marker)
}
