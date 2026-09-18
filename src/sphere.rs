use crate::caster::{Intersection, RayIntersect};
use crate::material::Material;
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
        if discriminant < 0.0 {
            // No intersection
            return Intersection::no_intersection();
        } else {
            Intersection::new(
                (-b - discriminant.sqrt()) / (2.0 * a),
                ray.origin + ray.direction * ((-b - discriminant.sqrt()) / (2.0 * a)),
                (ray.origin + ray.direction * ((-b - discriminant.sqrt()) / (2.0 * a)) - self.center).normalize(),
                true,
                self.material,
                (0.0, 0.0), // Placeholder for UV coordinates
                Vec3A::ZERO, // Placeholder for tangent
                Vec3A::ZERO, // Placeholder for bitangent
            )
        }
    }

    
}