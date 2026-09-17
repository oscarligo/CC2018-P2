use glam::Vec3A;
use raylib::prelude::Color;

#[derive(Clone, Debug, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: Color,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Intersection {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}


impl Intersection {
    pub fn new(distance: f32, is_intersecting: bool, material: Material) -> Self {
        Self {
            distance,
            is_intersecting,
            material,
        }
    }

    pub fn no_intersection() -> Self {
        Self {
            distance: f32::INFINITY,
            is_intersecting: false,
            material: Material {
                diffuse: Color::BLACK,
                specular: Color::BLACK,
            },
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

    for object in objects {
        let intersection = object.intersect(ray);
        if intersection.is_intersecting {
            return intersection.material.diffuse;
        }
    }

    Color::BLACK 
    
}
