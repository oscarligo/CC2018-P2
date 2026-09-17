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
        // Vector from the ray origin to the sphere center
        let oc = ray.origin - self.center;
        // Sphere intersection equation: (P - C) . (P - C) = r^2
        // Quadratic formula coefficients
        let a = ray.direction.dot(ray.direction);
        // Quadratic formula coefficients
        let b = 2.0 * oc.dot(ray.direction);  
        // Quadratic formula coefficients
        let c = oc.dot(oc) - self.radius * self.radius;

        // Discriminant of the quadratic equation
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }

    fn get_color(&self) -> Color {
        self.color
    }
}