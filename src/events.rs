use crate::player::Player;
use raylib::prelude::*;
use crate::maze::Maze;

const MOUSE_SENSITIVITY: f32 = 0.003;

pub fn process_events(
    window: &RaylibHandle,
    player: &mut Player,
    maze: &Maze,
    block_size: usize,

) {
    player.rotate(window.get_mouse_delta().x * MOUSE_SENSITIVITY);

    if window.is_key_down(KeyboardKey::KEY_W) {
        player.move_forward(maze, block_size);
    }

    if window.is_key_down(KeyboardKey::KEY_S) {
        player.move_backward(maze, block_size);
    }

    if window.is_key_down(KeyboardKey::KEY_A) {
        player.rotate_left();
    }

    if window.is_key_down(KeyboardKey::KEY_D) {
        player.rotate_right();
    }
}
