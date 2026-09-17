use crate::caster::{Intersection, Ray, RayIntersect};
use crate::material::Material;
use glam::Vec3A;

pub struct Cube {
    pub center: Vec3A,
    pub size: f32,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let half_size = self.size * 0.5;
        let extent = Vec3A::splat(half_size);
        let min_bound = self.center - extent;
        let max_bound = self.center + extent;

        let inv_dir = Vec3A::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let t1 = (min_bound - ray.origin) * inv_dir;
        let t2 = (max_bound - ray.origin) * inv_dir;

        let t_min = t1.min(t2);
        let t_max = t1.max(t2);

        let t_near = t_min.max_element();
        let t_far = t_max.min_element();

        if t_near > t_far || t_far < 0.001 {
            return Intersection::no_intersection();
        }

        let (t_hit, is_inside) = if t_near > 0.001 {
            (t_near, false)
        } else {
            (t_far, true)
        };

        let intersection_point = ray.origin + ray.direction * t_hit;

        let mut normal = if (t_near - t_min.x).abs() < 1e-4 {
            Vec3A::new(-ray.direction.x.signum(), 0.0, 0.0)
        } else if (t_near - t_min.y).abs() < 1e-4 {
            Vec3A::new(0.0, -ray.direction.y.signum(), 0.0)
        } else {
            Vec3A::new(0.0, 0.0, -ray.direction.z.signum())
        };

        if is_inside {
            normal = -normal;
        }

        Intersection::new(
            t_hit,
            intersection_point,
            normal,
            true,
            self.material,
        )
    }
}