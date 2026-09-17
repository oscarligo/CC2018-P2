use crate::caster::{Intersection, RayIntersect, Material};
use glam::Vec3A;
use crate::caster::Ray;

pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersection {
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
        Intersection {
            distance: if discriminant < 0.0 {
                f32::INFINITY
            } else {
                (-b - discriminant.sqrt()) / (2.0 * a)
            },
            is_intersecting: discriminant >= 0.0,
            material: self.material.clone(),
        }
    }

    
}