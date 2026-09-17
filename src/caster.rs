use glam::Vec3A;
use raylib::prelude::Color;

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}
pub trait RayIntersect{
    fn intersect(&self, ray: &Ray) -> bool; 
    fn get_color(&self) -> Color;
}

pub fn cast_ray(
    ray: &Ray,
    objects: &[Box<dyn RayIntersect>]

) -> Color {

    for object in objects {
        if object.intersect(ray) {
            return object.get_color();
        }
    }

    Color::BLACK 
    
}
