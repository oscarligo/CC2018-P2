use crate::camera::Camera;
use crate::render::RenderMode;
use crate::vector::Vec3A;
use raylib::prelude::*;

pub struct EventHandler {
    pub mouse_sensitivity: f32,
    pub zoom_speed: f32,
}

impl EventHandler {
    pub fn handle_events(
        &self,
        rl: &RaylibHandle,
        camera: &mut Camera,
        mode: &mut RenderMode,
    ) -> (bool, bool, (i32, i32)) {
        let mut changed = false;
        let mut camera_moving = false;

        // Render mode switching
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) && *mode != RenderMode::Flat {
            *mode = RenderMode::Flat;
            changed = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) && *mode != RenderMode::Normals {
            *mode = RenderMode::Normals;
            changed = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) && *mode != RenderMode::BasicLight {
            *mode = RenderMode::BasicLight;
            changed = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) && *mode != RenderMode::Full {
            *mode = RenderMode::Full;
            changed = true;
        }

        // Terrain shifting
        let mut shift_x = 0;
        let mut shift_z = 0;

        if rl.is_key_pressed(KeyboardKey::KEY_W) {
            shift_z -= 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            shift_z += 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_A) {
            shift_x -= 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_D) {
            shift_x += 1;
        }

        let terrain_moved = shift_x != 0 || shift_z != 0;

        // Camera movement
        let lmb_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let rmb_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT);
        let mouse_delta = rl.get_mouse_delta();

        if rmb_down && (mouse_delta.x != 0.0 || mouse_delta.y != 0.0) {
            camera.pan(mouse_delta.x, mouse_delta.y);
            camera_moving = true;
        } else if lmb_down && (mouse_delta.x != 0.0 || mouse_delta.y != 0.0) {
            let yaw = -mouse_delta.x * self.mouse_sensitivity;
            let pitch = -mouse_delta.y * self.mouse_sensitivity;
            camera.orbit(yaw, pitch);
            camera_moving = true;
        }

        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            camera.zoom(wheel * self.zoom_speed);
            camera_moving = true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            camera.center = Vec3A::ZERO;
            camera.update_basis_vectors();
            camera_moving = true;
        }

        let camera_changed = camera.is_changed();
        let total_changed = changed || camera_changed || terrain_moved;

        (total_changed, camera_moving, (shift_x, shift_z))
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.005,
            zoom_speed: 0.15,
        }
    }
}
