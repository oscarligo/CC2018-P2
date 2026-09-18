use raylib::prelude::*;
use glam::Vec3A;
use crate::camera::Camera;
use crate::render::RenderMode;

pub struct EventHandler {
    pub mouse_sensitivity: f32,
    pub zoom_speed: f32,
}

impl EventHandler {
    pub fn new(mouse_sensitivity: f32, zoom_speed: f32) -> Self {
        Self {
            mouse_sensitivity,
            zoom_speed,
        }
    }

    pub fn handle_events(
        &self,
        rl: &RaylibHandle,
        camera: &mut Camera,
        mode: &mut RenderMode,
    ) -> bool {
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

        let lmb_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let rmb_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT);

        let mouse_delta = rl.get_mouse_delta();

        if  rmb_down {
            if mouse_delta.x != 0.0 || mouse_delta.y != 0.0 {
                camera.pan(mouse_delta.x, mouse_delta.y);
                changed = true;
            }
        }
        
        else if lmb_down {
            if mouse_delta.x != 0.0 || mouse_delta.y != 0.0 {
                let yaw = -mouse_delta.x * self.mouse_sensitivity;
                let pitch = -mouse_delta.y * self.mouse_sensitivity;
                camera.orbit(yaw, pitch);
                changed = true;
            }
        }

        // 5. ZOOM CON RUEDA
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            camera.zoom(wheel * self.zoom_speed);
            changed = true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            camera.center = Vec3A::ZERO;
            camera.update_basis_vectors();
            changed = true;
        }

        changed || camera.is_changed()
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