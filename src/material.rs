use glam::Vec3A;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Material {
    pub diffuse_color: Vec3A,   // Base color
    pub albedo: [f32; 4],       // Reflectivity coefficients: [diffuse, specular, reflection, refraction]
    pub specular_exponent: f32, // Shininess factor for specular highlights
    pub refractive_index: f32,  // Refractive index for transparent materials
}

impl Material {
    
    // Constructor for Material
    #[inline(always)]
    pub const fn new(
        diffuse_color: Vec3A,
        albedo: [f32; 4],
        specular_exponent: f32,
        refractive_index: f32,
    ) -> Self {
        Self {
            diffuse_color,
            albedo,
            specular_exponent,
            refractive_index,
        }
    }

    // Constructor for an empty material (default)
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            diffuse_color: Vec3A::ZERO,
            albedo: [0.0, 0.0, 0.0, 0.0],
            specular_exponent: 0.0,
            refractive_index: 1.0,
        }
    }

    // Constructor for a simple diffuse material
    #[inline(always)]
    pub fn diffuse(color: Vec3A) -> Self {
        Self {
            diffuse_color: color,
            albedo: [1.0, 0.0, 0.0, 0.0],
            specular_exponent: 1.0,
            refractive_index: 1.0,
        }
    }

    // Constructor for a plastic material with specular reflection
    #[inline(always)]
    pub fn plastic(color: Vec3A) -> Self {
        Self {
            diffuse_color: color,
            albedo: [0.7, 0.3, 0.0, 0.0],
            specular_exponent: 50.0,
            refractive_index: 1.0,
        }
    }

    // Constructor for a mirror material with reflection
    #[inline(always)]
    pub fn mirror(reflection: f32) -> Self {
        Self {
            diffuse_color: Vec3A::splat(1.0),
            albedo: [0.0, 10.0, reflection.clamp(0.0, 1.0), 0.0],
            specular_exponent: 1425.0,
            refractive_index: 1.0,
        }
    }

    // Constructor for a glass material with refraction and slight reflection
    #[inline(always)]
    pub fn glass(refractive_index: f32) -> Self {
        Self {
            diffuse_color: Vec3A::splat(1.0),
            albedo: [0.0, 0.5, 0.1, 0.8],
            specular_exponent: 125.0,
            refractive_index,
        }
    }
}
