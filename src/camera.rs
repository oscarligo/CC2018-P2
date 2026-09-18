use glam::{Quat, Vec3A};

pub struct Camera {
    pub eye: Vec3A,
    pub center: Vec3A,
    pub up: Vec3A,
    pub forward: Vec3A,
    pub right: Vec3A,
    changed: bool,
}

impl Camera {
    pub fn new(eye: Vec3A, center: Vec3A, up: Vec3A) -> Self {
        let forward = (center - eye).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward).normalize();

        Self {
            eye,
            center,
            up,
            forward,
            right,
            changed: true,
        }
    }

    pub fn update_basis_vectors(&mut self) {
        self.forward = (self.center - self.eye).normalize();
        let world_up = Vec3A::Y;
        let right = self.forward.cross(world_up);
    
        self.right = if right.length_squared() > 1e-6 {
            right.normalize()
        } else {
            self.right
        };
        
        self.up = self.right.cross(self.forward).normalize();
        self.changed = true;
    }

    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        let mut offset = self.eye - self.center;

        let q_yaw = Quat::from_axis_angle(Vec3A::Y.into(), yaw);
        offset = q_yaw * offset;

        let current_dir = offset.normalize();
        let up_dot = current_dir.dot(Vec3A::Y);
        
        let safe_pitch = if (up_dot > 0.98 && pitch < 0.0) || (up_dot < -0.98 && pitch > 0.0) {
            0.0
        } else {
            pitch
        };

        let q_pitch = Quat::from_axis_angle(self.right.into(), safe_pitch);
        offset = q_pitch * offset;

        self.eye = self.center + offset;
        self.update_basis_vectors();
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let dist = (self.eye - self.center).length();
        let factor = (dist * 0.002).max(0.01);

        let translation = (-self.right * delta_x + self.up * delta_y) * factor;
        self.eye += translation;
        self.center += translation;
        self.changed = true;
    }

    pub fn zoom(&mut self, amount: f32) {
        let to_center = self.center - self.eye;
        let dist = to_center.length();

        let delta = dist * amount;

        if dist - delta > 0.05 {
            self.eye += to_center.normalize() * delta;
        } else {
            let forward = to_center.normalize();
            self.eye += forward * delta;
            self.center += forward * delta;
        }

        self.update_basis_vectors();
    }

    pub fn is_changed(&mut self) -> bool {
        let changed = self.changed;
        self.changed = false;
        changed
    }

    #[inline(always)]
    pub fn basis_change(&self, v: &Vec3A) -> Vec3A {
        Vec3A::new(
            v.x * self.right.x + v.y * self.up.x - v.z * self.forward.x,
            v.x * self.right.y + v.y * self.up.y - v.z * self.forward.y,
            v.x * self.right.z + v.y * self.up.z - v.z * self.forward.z,
        )
    }
}