use crate::caster::{Intersection, Ray, RayIntersect};
use crate::material::Material;
use crate::vector::Vec3A;

#[derive(Clone, Copy)]
pub struct Cube {
    pub center: Vec3A,
    pub material: Material,
    pub min_b: Vec3A,
    pub max_b: Vec3A,
    pub inv_half: f32,
}

impl Cube {
    pub fn new(center: Vec3A, size: f32, material: Material) -> Self {
        let half = size * 0.5;
        let min_b = center - Vec3A::splat(half);
        let max_b = center + Vec3A::splat(half);
        let inv_half = 1.0 / half;

        Self {
            center,
            material,
            min_b,
            max_b,
            inv_half,
        }
    }
}

impl RayIntersect for Cube {
    #[inline(always)]
    fn intersect(&self, ray: &Ray) -> Intersection {
        let t1 = (self.min_b - ray.origin) * ray.inv_direction;
        let t2 = (self.max_b - ray.origin) * ray.inv_direction;

        let t_min = t1.min(t2);
        let t_max = t1.max(t2);

        let t_near = t_min.max_element();
        let t_far = t_max.min_element();

        if t_near > t_far || t_far < 0.001 {
            return Intersection::no_intersection();
        }

        let is_outside = t_near > 0.001;
        let t_hit = if is_outside { t_near } else { t_far };
        let t_target = if is_outside { t_min } else { t_max };

        let hit_p = ray.origin + ray.direction * t_hit;
        let local_p = (hit_p - self.center) * self.inv_half;

        let (normal, u, v, tangent, bitangent) = if t_hit == t_target.x {
            let sign = if is_outside { -ray.direction.x.signum() } else { ray.direction.x.signum() };
            (
                Vec3A::new(sign, 0.0, 0.0),
                (-local_p.z * sign + 1.0) * 0.5,
                (1.0 - local_p.y) * 0.5,
                Vec3A::new(0.0, 0.0, -sign),
                Vec3A::new(0.0, -1.0, 0.0),
            )
        } else if t_hit == t_target.y {
            let sign = if is_outside { -ray.direction.y.signum() } else { ray.direction.y.signum() };
            (
                Vec3A::new(0.0, sign, 0.0),
                (local_p.x + 1.0) * 0.5,
                (local_p.z * sign + 1.0) * 0.5,
                Vec3A::new(1.0, 0.0, 0.0),
                Vec3A::new(0.0, 0.0, sign),
            )
        } else {
            // Eje Z
            let sign = if is_outside { -ray.direction.z.signum() } else { ray.direction.z.signum() };
            (
                Vec3A::new(0.0, 0.0, sign),
                (local_p.x * sign + 1.0) * 0.5,
                (1.0 - local_p.y) * 0.5,
                Vec3A::new(sign, 0.0, 0.0),
                Vec3A::new(0.0, -1.0, 0.0),
            )
        };

        Intersection::new(
            t_hit,
            hit_p,
            normal,
            true,
            self.material,
            (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)),
            tangent,
            bitangent,
        )
    }
}
