use crate::caster::{Intersection, Ray, RayIntersect};
use crate::material::Material;
use glam::Vec3A;


#[derive(Clone, Copy)]
pub struct Cube {
    pub center: Vec3A,
    pub size: f32,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let half = self.size * 0.5;
        let min_b = self.center - Vec3A::splat(half);
        let max_b = self.center + Vec3A::splat(half);

        let inv_d = Vec3A::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        let t1 = (min_b - ray.origin) * inv_d;
        let t2 = (max_b - ray.origin) * inv_d;

        let t_min = t1.min(t2);
        let t_max = t1.max(t2);

        let t_near = t_min.max_element();
        let t_far = t_max.min_element();

        if t_near > t_far || t_far < 0.001 {
            return Intersection::no_intersection();
        }

        let t_hit = if t_near > 0.001 { t_near } else { t_far };
        let hit_p = ray.origin + ray.direction * t_hit;

        let normal = if (t_near - t_min.x).abs() < 1e-4 {
            Vec3A::new(-ray.direction.x.signum(), 0.0, 0.0)
        } else if (t_near - t_min.y).abs() < 1e-4 {
            Vec3A::new(0.0, -ray.direction.y.signum(), 0.0)
        } else {
            Vec3A::new(0.0, 0.0, -ray.direction.z.signum())
        };

        let local_p = (hit_p - self.center) / half;
        let u: f32;
        let v: f32;
        let tangent: Vec3A;
        let bitangent: Vec3A;

        if normal.x.abs() > 0.9 {
            let sign = normal.x.signum();
            u = (-local_p.z * sign + 1.0) * 0.5;
            v = (1.0 - local_p.y) * 0.5;
            tangent = Vec3A::new(0.0, 0.0, -sign);
            bitangent = Vec3A::new(0.0, -1.0, 0.0);
        } else if normal.y.abs() > 0.9 {
            let sign = normal.y.signum();
            u = (local_p.x + 1.0) * 0.5;
            v = (local_p.z * sign + 1.0) * 0.5;
            tangent = Vec3A::new(1.0, 0.0, 0.0);
            bitangent = Vec3A::new(0.0, 0.0, sign);
        } else {
            let sign = normal.z.signum();
            u = (local_p.x * sign + 1.0) * 0.5;
            v = (1.0 - local_p.y) * 0.5;
            tangent = Vec3A::new(sign, 0.0, 0.0);
            bitangent = Vec3A::new(0.0, -1.0, 0.0);
        }

        Intersection::new(
            t_hit,
            hit_p,
            normal,
            true,
            self.material, // Copy puro
            (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)),
            tangent,
            bitangent,
        )
    }
}