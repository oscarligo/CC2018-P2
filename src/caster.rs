use glam::Vec3A;
use crate::material::*;
use crate::render::RenderMode;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Intersection {
    pub distance: f32,
    pub point: Vec3A,
    pub normal: Vec3A,
    pub is_intersecting: bool,
    pub material: Material,
}


impl Intersection {
    pub fn new(distance: f32, point: Vec3A, normal: Vec3A, is_intersecting: bool, material: Material) -> Self {
        Self {
            distance,
            point,
            normal,
            is_intersecting,
            material,
        }
    }

    pub fn no_intersection() -> Self {
        Self {
            distance: f32::INFINITY,
            point: Vec3A::ZERO,
            normal: Vec3A::ZERO,
            is_intersecting: false,
            material: Material::empty(),
        }
    }
}

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}

pub trait RayIntersect{
    fn intersect(&self, ray: &Ray) -> Intersection; 
}

pub fn cast_ray(
    ray: &Ray,
    objects: &[impl RayIntersect + Sync],
    lights: &[Light],
    mode: RenderMode, 
) -> Vec3A {
    let mut intersection = Intersection::no_intersection();
    let mut z_buffer = f32::INFINITY;

    for object in objects {
        let tmp_intersection = object.intersect(ray);
        if tmp_intersection.is_intersecting
            && tmp_intersection.distance < z_buffer
            && tmp_intersection.distance > 0.001
        {
            intersection = tmp_intersection;
            z_buffer = tmp_intersection.distance;
        }
    }

    if !intersection.is_intersecting {
        return Vec3A::new(0.2, 0.7, 0.8); 
    }

    match mode {
        RenderMode::Flat => intersection.material.diffuse_color,

        RenderMode::Normals => (intersection.normal + Vec3A::splat(1.0)) * 0.5,

        RenderMode::BasicLight => {
            let mut light_sum = 0.1;
            for light in lights {
                let light_dir = (light.position - intersection.point).normalize();
                let n_dot_l = intersection.normal.dot(light_dir).max(0.0);
                light_sum += n_dot_l * light.intensity;
            }
            intersection.material.diffuse_color * light_sum
        }

        
        RenderMode::Full => cast_ray_recursive(ray, objects, lights, 0, &intersection),
    }
}


fn cast_ray_recursive(
    ray: &Ray,
    objects: &[impl RayIntersect + Sync],
    lights: &[Light],
    depth: u32,
    hit: &Intersection,
) -> Vec3A {
    if depth > 2 {
        return Vec3A::ZERO;
    }

    hit.material.shade(
        ray,
        hit,
        objects,
        lights,
        depth,
        &|secondary_ray, objs, lts, next_depth| {
            let mut sec_hit = Intersection::no_intersection();
            let mut sec_z = f32::INFINITY;

            for obj in objs {
                let tmp = obj.intersect(secondary_ray);
                if tmp.is_intersecting && tmp.distance < sec_z && tmp.distance > 0.001 {
                    sec_z = tmp.distance;
                    sec_hit = tmp;
                }
            }

            if !sec_hit.is_intersecting {
                return Vec3A::new(0.2, 0.7, 0.8);
            }

            cast_ray_recursive(secondary_ray, objs, lts, next_depth, &sec_hit)
        },
    )
}