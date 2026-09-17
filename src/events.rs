use raylib::prelude::*;
use crate::camera::Camera;
use crate::render::RenderMode;

pub struct EventHandler {
    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub mouse_sensitivity: f32,
}

impl EventHandler {
    pub fn new(rotation_speed: f32, zoom_speed: f32, mouse_sensitivity: f32) -> Self {
        Self {
            rotation_speed,
            zoom_speed,
            mouse_sensitivity,
        }
    }


    pub fn handle_events(
        &self,
        rl: &RaylibHandle,
        camera: &mut Camera,
        mode: &mut RenderMode,
    ) -> bool {
        let delta = rl.get_frame_time();
        let mut changed = false;

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

        // 2. Rotación con teclado (WASD o Flechas)
        let rot_amount = self.rotation_speed * delta;
        let mut yaw = 0.0;
        let mut pitch = 0.0;

        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            yaw -= rot_amount;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            yaw += rot_amount;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
            pitch += rot_amount;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
            pitch -= rot_amount;
        }

        // 3. Rotación con ratón (arrastrar con clic izquierdo)
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_delta = rl.get_mouse_delta();
            yaw += mouse_delta.x * self.mouse_sensitivity;
            pitch += -mouse_delta.y * self.mouse_sensitivity;
        }

        if yaw != 0.0 || pitch != 0.0 {
            camera.orbit(yaw, pitch);
            changed = true;
        }

        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            camera.zoom(wheel * self.zoom_speed);
            changed = true;
        }

        changed || camera.is_changed()
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new(2.0, 0.5, 0.005)
    }
}