use glam::Vec3A;
use raylib::prelude::Color;
use crate::material::Material;

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
    objects: &[impl RayIntersect]

) -> Color {

    let mut intersection = Intersection::no_intersection();
    let mut z_buffer = f32::INFINITY;

    for object in objects {
        let tmp_intersection = object.intersect(ray);
        if tmp_intersection.is_intersecting && tmp_intersection.distance < z_buffer {
            intersection = tmp_intersection;
            z_buffer = tmp_intersection.distance;
        }
    }

    if !intersection.is_intersecting {
        return Color::BLACK;
    } 

    return Color::new(
        (intersection.material.diffuse_color.x * 255.0) as u8,
        (intersection.material.diffuse_color.y * 255.0) as u8,
        (intersection.material.diffuse_color.z * 255.0) as u8,
        255,
    );

    
}
