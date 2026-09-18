use glam::Vec3A;
use crate::material::*;
use crate::render::RenderMode;
use crate::background::BackgroundTexture;
use crate::texture::Texture;


/*
    This module defines the core raycasting logic for the 3D scene.
    It includes the Intersection struct, Ray struct, and the RayIntersect trait.
    The cast_ray function handles ray-object intersections and shading based on 
    the selected render mode.
*/

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Intersection {
    pub distance: f32, // Distance from the ray origin to the intersection point
    pub point: Vec3A, // Intersection point in world space
    pub normal: Vec3A, // Surface normal at the intersection point
    pub is_intersecting: bool, // Flag indicating whether an intersection occurred
    pub material: Material, // Material properties at the intersection point
    pub uv: (f32, f32), // UV texture coordinates at the intersection point
    pub tangent: Vec3A, // Tangent vector at the intersection point (for normal mapping)
    pub bitangent: Vec3A, // Bitangent vector at the intersection point (for normal mapping)
}

impl Intersection {

    // Creates a new Intersection instance with the provided parameters.
    #[inline(always)]
    pub fn new(
        distance: f32,
        point: Vec3A,
        normal: Vec3A,
        is_intersecting: bool,
        material: Material,
        uv: (f32, f32),
        tangent: Vec3A,
        bitangent: Vec3A,
    ) -> Self {
        Self {
            distance,
            point,
            normal,
            is_intersecting,
            material,
            uv,
            tangent,
            bitangent,
        }
    }

    // Returns an Intersection instance representing no intersection.
    #[inline(always)]
    pub fn no_intersection() -> Self {
        Self {
            distance: f32::INFINITY,
            point: Vec3A::ZERO,
            normal: Vec3A::ZERO,
            is_intersecting: false,
            material: Material::empty(),
            uv: (0.0, 0.0),
            tangent: Vec3A::ZERO,
            bitangent: Vec3A::ZERO,
        }
    }
}

// Represents a ray in 3D space with an origin and direction.
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}

// Defines the interface for objects that can be intersected by a ray.
pub trait RayIntersect: Sync + Send {
    fn intersect(&self, ray: &Ray) -> Intersection; 
}

// Computes the perturbed normal at the intersection point using the normal map, if available.
#[inline(always)]
fn get_perturbed_normal(hit: &Intersection, textures: &[Texture]) -> Vec3A {
    if let Some(norm_id) = hit.material.textures.normal_id {
        let raw = textures[norm_id].sample(hit.uv.0, hit.uv.1);
        let tan_n = (raw * 2.0 - Vec3A::splat(1.0)).normalize();
        (hit.tangent * tan_n.x + hit.bitangent * tan_n.y + hit.normal * tan_n.z).normalize()
    } else {
        hit.normal
    }
}

// Casts a ray into the scene, checking for intersections with objects and applying 
// shading based on the render mode.
pub fn cast_ray(
    ray: &Ray,
    objects: &[impl RayIntersect + Sync],
    lights: &[Light],
    mode: RenderMode, 
    background: &BackgroundTexture,
    textures: &[Texture],
) -> Vec3A {
    let mut intersection = Intersection::no_intersection();
    let mut z_buffer = f32::INFINITY;

    for object in objects {
        let tmp = object.intersect(ray);
        if tmp.is_intersecting && tmp.distance < z_buffer && tmp.distance > 0.001 {
            z_buffer = tmp.distance;
            intersection = tmp;
        }
    }

    if !intersection.is_intersecting {
        return background.sample(&ray.direction);
    }

    // Handle shading based on the selected render mode
    match mode {
        
        // Flat shading mode: returns the base color of the material or the diffuse texture color if available
        RenderMode::Flat => {
            if let Some(diff_id) = intersection.material.textures.diffuse_id {
                textures[diff_id].sample(intersection.uv.0, intersection.uv.1)
            } else {
                intersection.material.diffuse_color
            }
        }

        // Normal shading mode: displays the normal map in RGB (tangible relief)
        RenderMode::Normals => {
            let n = get_perturbed_normal(&intersection, textures);
            (n + Vec3A::splat(1.0)) * 0.5
        }

        // Basic lighting mode: applies simple diffuse lighting without shadows
        RenderMode::BasicLight => {
            let n = get_perturbed_normal(&intersection, textures);
            let mut light_sum = 0.1;
            for light in lights {
                let light_dir = (light.position - intersection.point).normalize();
                let n_dot_l = n.dot(light_dir).max(0.0);
                light_sum += n_dot_l * light.intensity;
            }
            let color = if let Some(diff_id) = intersection.material.textures.diffuse_id {
                textures[diff_id].sample(intersection.uv.0, intersection.uv.1)
            } else {
                intersection.material.diffuse_color
            };
            color * light_sum
        }

        // Full shading mode: applies full lighting with shadows, reflections, and refractions
        RenderMode::Full => cast_ray_recursive(ray, objects, lights, background, textures, 0, &intersection),
    }
}



// Recursively casts a ray through the scene, handling reflections and refractions.
fn cast_ray_recursive(
    ray: &Ray,
    objects: &[impl RayIntersect + Sync],
    lights: &[Light],
    background: &BackgroundTexture,
    textures: &[Texture],
    depth: u32,
    hit: &Intersection,
) -> Vec3A {

    // Limit recursion depth to avoid infinite loops and excessive computation
    if depth > 3 {
        return Vec3A::ZERO;
    }

    let (u, v) = hit.uv; // UV coordinates for texture sampling


    // Diffuse color from the material or texture
    let base_diffuse = if let Some(diff_id) = hit.material.textures.diffuse_id {
        textures[diff_id].sample(u, v) * hit.material.diffuse_color
    } else {
        hit.material.diffuse_color
    };

    // Perturbed normal for shading, taking into account the normal map if available
    let shading_normal = get_perturbed_normal(hit, textures);

    // Specular factor from the specular map if available, otherwise default to 1.0
    let specular_factor = if let Some(spec_id) = hit.material.textures.specular_id {
        textures[spec_id].sample(u, v).x
    } else {
        1.0
    };

    let mut diffuse_light = Vec3A::ZERO;
    let mut specular_light = Vec3A::ZERO;

    // Only calculate shadows and direct lighting if it has a diffuse or specular component
    if hit.material.albedo[0] > 0.0 || hit.material.albedo[1] > 0.0 {
        for light in lights {
            let light_vec = light.position - hit.point;
            let light_dist = light_vec.length();
            let light_dir = light_vec / light_dist;

            // Offset the shadow ray origin to avoid self-intersection artifacts
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

            let n_dot_l = shading_normal.dot(light_dir).max(0.0);
            diffuse_light += light.color * (light.intensity * n_dot_l);

            let reflect_dir = reflect(-light_dir, shading_normal);
            let view_dir = -ray.direction;
            let spec = view_dir.dot(reflect_dir).max(0.0).powf(hit.material.specular_exponent);
            specular_light += light.color * (light.intensity * spec * specular_factor);
        }
    }

    // Reflection 
    let mut reflection_color = Vec3A::ZERO; // Reflection color initialized to zero

    // Only calculate reflection if the material has a reflective component
    if hit.material.albedo[2] > 0.0 {
        let reflect_dir = reflect(ray.direction, shading_normal).normalize();
        let reflect_orig = if reflect_dir.dot(hit.normal) < 0.0 {
            hit.point - hit.normal * 0.001
        } else {
            hit.point + hit.normal * 0.001
        };
        let reflect_ray = Ray {
            origin: reflect_orig,
            direction: reflect_dir,
        };

        let mut sec_hit = Intersection::no_intersection();
        let mut sec_z = f32::INFINITY;
        for obj in objects {
            let tmp = obj.intersect(&reflect_ray);
            if tmp.is_intersecting && tmp.distance < sec_z && tmp.distance > 0.001 {
                sec_z = tmp.distance;
                sec_hit = tmp;
            }
        }

        reflection_color = if sec_hit.is_intersecting {
            cast_ray_recursive(&reflect_ray, objects, lights, background, textures, depth + 1, &sec_hit)
        } else {
            background.sample(&reflect_ray.direction)
        };
    }

    // Refraction
    let mut refraction_color = Vec3A::ZERO;
    // Only calculate refraction if the material has a refractive component
    if hit.material.albedo[3] > 0.0 {
        let kr = fresnel(ray.direction, shading_normal, hit.material.refractive_index);

        if let Some(refract_dir) = refract(ray.direction, shading_normal, hit.material.refractive_index) {
            let refract_orig = if refract_dir.dot(hit.normal) < 0.0 {
                hit.point - hit.normal * 0.001
            } else {
                hit.point + hit.normal * 0.001
            };
            let refract_ray = Ray {
                origin: refract_orig,
                direction: refract_dir,
            };

            let mut sec_hit = Intersection::no_intersection();
            let mut sec_z = f32::INFINITY;
            for obj in objects {
                let tmp = obj.intersect(&refract_ray);
                if tmp.is_intersecting && tmp.distance < sec_z && tmp.distance > 0.001 {
                    sec_z = tmp.distance;
                    sec_hit = tmp;
                }
            }

            let transmitted_color = if sec_hit.is_intersecting {
                cast_ray_recursive(&refract_ray, objects, lights, background, textures, depth + 1, &sec_hit)
            } else {
                background.sample(&refract_ray.direction)
            };

            // Modulación de Fresnel: reflectancia vs transmitancia
            refraction_color = transmitted_color * (1.0 - kr) + reflection_color * kr;
        } else {
            // Reflexión interna total (TIR): toda la energía se convierte en reflejo
            refraction_color = reflection_color;
        }
    }

    let ambient = base_diffuse * 0.05;

    ambient
        + (base_diffuse * diffuse_light * hit.material.albedo[0])
        + (Vec3A::splat(1.0) * specular_light * hit.material.albedo[1])
        + (reflection_color * hit.material.albedo[2])
        + (refraction_color * hit.material.albedo[3])
}

// Calculates the reflection direction of a ray given its incoming direction 
//and the surface normal.
#[inline(always)]
fn reflect(dir: Vec3A, normal: Vec3A) -> Vec3A {
    dir - normal * 2.0 * dir.dot(normal)
}

// Calculates the refraction direction of a ray given its incoming direction,
// the surface normal, and the material's refractive index.
#[inline(always)]
fn refract(dir: Vec3A, normal: Vec3A, refractive_index: f32) -> Option<Vec3A> {
    let mut cosi = -dir.dot(normal).clamp(-1.0, 1.0);
    let mut n = normal;
    let mut eta_i = 1.0;
    let mut eta_t = refractive_index;

    // Swap the indices of refraction if the ray is exiting the material
    if cosi < 0.0 {
        cosi = -cosi;
        n = -normal;
        std::mem::swap(&mut eta_i, &mut eta_t);
    }

    let eta = eta_i / eta_t;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    // If k < 0, it means total internal reflection occurs, and no refraction is possible.
    if k < 0.0 {
        None
    } else {
        Some((dir * eta + n * (eta * cosi - k.sqrt())).normalize())
    }
}

// Computes the Fresnel effect, which determines how much light is reflected vs refracted
#[inline(always)]
fn fresnel(dir: Vec3A, normal: Vec3A, refractive_index: f32) -> f32 {
    let mut cosi = -dir.dot(normal).clamp(-1.0, 1.0);
    let mut eta_i = 1.0;
    let mut eta_t = refractive_index;

    // if the ray is exiting the material, swap the indices of refraction
    if cosi < 0.0 {
        cosi = -cosi;
        std::mem::swap(&mut eta_i, &mut eta_t);
    }

    let r0 = ((eta_i - eta_t) / (eta_i + eta_t)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosi).powi(5)
}