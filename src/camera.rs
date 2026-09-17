use glam::Vec3A;

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
        self.right = self.forward.cross(self.up).normalize();
        self.up = self.right.cross(self.forward).normalize();
        self.changed = true;
    }



    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        let relative_pos = self.eye - self.center;
        let radius = relative_pos.length();
        let current_yaw = relative_pos.z.atan2(relative_pos.x);
        let current_pitch = (relative_pos.y / radius).asin();
        
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(-1.5, 1.5); 
        let cos_pitch = new_pitch.cos();
        let sin_pitch = new_pitch.sin();
        let new_relative_pos = Vec3A::new(
            radius * cos_pitch * new_yaw.cos(),  
            radius * sin_pitch,                  
            radius * cos_pitch * new_yaw.sin(),  
        );
        self.eye = self.center + new_relative_pos;
        self.update_basis_vectors();
    }

    pub fn zoom(&mut self, amount: f32) {
        let forward = (self.center - self.eye).normalize();
        self.eye += forward * amount;
        self.update_basis_vectors();
    }

    pub fn is_changed(&mut self) -> bool {
        let changed = self.changed;
        self.changed = false;
        changed
    }

    pub fn basis_change(&self, v: &Vec3A) -> Vec3A {
        Vec3A::new(
            v.x * self.right.x + v.y * self.up.x - v.z * self.forward.x,
            v.x * self.right.y + v.y * self.up.y - v.z * self.forward.y,
            v.x * self.right.z + v.y * self.up.z - v.z * self.forward.z,
        )
    }
}