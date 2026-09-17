use crate::caster::RayIntersect;
use glam::Vec3A;
use crate::caster::Ray;
use raylib::prelude::Color;


pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
    pub color: Color,
}

impl RayIntersect for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // Cuatradic coefficients
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);  

        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }

    fn get_color(&self) -> Color {
        self.color
    }
}