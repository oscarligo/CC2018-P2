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
    ) -> (bool, bool, (f32, f32), bool, i32, bool) {
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

        if rl.is_key_down(KeyboardKey::KEY_W) {
            shift_z -= 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            shift_z += 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            shift_x -= 1;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            shift_x += 1;
        }
        let jump_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        let block_selection = i32::from(rl.is_key_pressed(KeyboardKey::KEY_RIGHT))
            - i32::from(rl.is_key_pressed(KeyboardKey::KEY_LEFT));
        let place_block = rl.is_key_pressed(KeyboardKey::KEY_P);

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

        let movement = if shift_x != 0 || shift_z != 0 {
            let forward = Vec3A::new(camera.forward.x, 0.0, camera.forward.z).normalize();
            let right = Vec3A::new(camera.right.x, 0.0, camera.right.z).normalize();
            let direction = right * shift_x as f32 - forward * shift_z as f32;
            let direction = direction.normalize();
            (direction.x, direction.z)
        } else {
            (0.0, 0.0)
        };

        let camera_changed = camera.is_changed();
        let total_changed = changed || camera_changed;

        (
            total_changed,
            camera_moving,
            movement,
            jump_pressed,
            block_selection,
            place_block,
        )
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
