use glam::Vec3A;
use crate::caster::{Intersection, Ray, RayIntersect};

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Material {
    pub diffuse_color: Vec3A,   // Base color (RGB en [0.0, 1.0])
    pub albedo: [f32; 4],       // Coeficientes: [difuso, especular, reflexión, refracción]
    pub specular_exponent: f32, // Exponente de brillo especular
    pub refractive_index: f32,  // Índice de refracción
}

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub position: Vec3A,
    pub intensity: f32,
    pub color: Vec3A,
}

impl Material {
    pub fn empty() -> Self {
        Self {
            diffuse_color: Vec3A::ZERO,
            albedo: [0.0; 4],
            specular_exponent: 0.0,
            refractive_index: 1.0,
        }
    }

    pub fn shade<T: RayIntersect + Sync>(
        &self,
        ray: &Ray,
        hit: &Intersection,
        objects: &[T],
        lights: &[Light],
        depth: u32,
        cast_ray_fn: &impl Fn(&Ray, &[T], &[Light], u32) -> Vec3A,
    ) -> Vec3A {
        if depth > 3 {
            return Vec3A::ZERO;
        }

        let mut diffuse_light = Vec3A::ZERO;
        let mut specular_light = Vec3A::ZERO;

        for light in lights {
            let light_vec = light.position - hit.point;
            let light_dist = light_vec.length();
            let light_dir = light_vec / light_dist;

            let shadow_orig = if light_dir.dot(hit.normal) < 0.0 {
                hit.point - hit.normal * 0.001
            } else {
                hit.point + hit.normal * 0.001
            };
            let shadow_ray = Ray {
                origin: shadow_orig,
                direction: light_dir,
            };

            let in_shadow = objects.iter().any(|obj| {
                let s = obj.intersect(&shadow_ray);
                s.is_intersecting && s.distance < light_dist && s.distance > 0.001
            });

            if in_shadow {
                continue;
            }

            let n_dot_l = hit.normal.dot(light_dir).max(0.0);
            diffuse_light += light.color * (light.intensity * n_dot_l);

            let reflect_dir = reflect(-light_dir, hit.normal);
            let view_dir = -ray.direction;
            let spec_factor = view_dir.dot(reflect_dir).max(0.0).powf(self.specular_exponent);
            specular_light += light.color * (light.intensity * spec_factor);
        }

        let mut reflection_color = Vec3A::ZERO;
        if self.albedo[2] > 0.0 {
            let reflect_dir = reflect(ray.direction, hit.normal).normalize();
            let reflect_orig = if reflect_dir.dot(hit.normal) < 0.0 {
                hit.point - hit.normal * 0.001
            } else {
                hit.point + hit.normal * 0.001
            };
            let reflect_ray = Ray {
                origin: reflect_orig,
                direction: reflect_dir,
            };
            reflection_color = cast_ray_fn(&reflect_ray, objects, lights, depth + 1);
        }

        let mut refraction_color = Vec3A::ZERO;
        if self.albedo[3] > 0.0 {
            if let Some(refract_dir) = refract(ray.direction, hit.normal, self.refractive_index, 1.0) {
                let refract_orig = if refract_dir.dot(hit.normal) < 0.0 {
                    hit.point - hit.normal * 0.001
                } else {
                    hit.point + hit.normal * 0.001
                };
                let refract_ray = Ray {
                    origin: refract_orig,
                    direction: refract_dir.normalize(),
                };
                refraction_color = cast_ray_fn(&refract_ray, objects, lights, depth + 1);
            }
        }

        let ambient = self.diffuse_color * 0.05;

        ambient
            + (self.diffuse_color * diffuse_light * self.albedo[0])
            + (Vec3A::splat(1.0) * specular_light * self.albedo[1])
            + (reflection_color * self.albedo[2])
            + (refraction_color * self.albedo[3])
    }
}

#[inline(always)]
fn reflect(dir: Vec3A, normal: Vec3A) -> Vec3A {
    dir - normal * 2.0 * dir.dot(normal)
}

#[inline(always)]
fn refract(dir: Vec3A, normal: Vec3A, eta_t: f32, eta_i: f32) -> Option<Vec3A> {
    let mut cosi = -dir.dot(normal).clamp(-1.0, 1.0);
    let mut n = normal;
    let mut eta = eta_i / eta_t;

    if cosi < 0.0 {
        cosi = -cosi;
        n = -normal;
        eta = eta_t / eta_i;
    }

    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        None
    } else {
        Some(dir * eta + n * (eta * cosi - k.sqrt()))
    }
}