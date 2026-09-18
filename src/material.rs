use glam::Vec3A;
/*
    This struct holds the IDs of the textures used by a material.
    Each ID corresponds to a specific type of texture (diffuse, normal, specular).
*/
#[derive(Clone, Copy, Debug)]
pub struct MaterialTextureIds {
    pub diffuse_id: Option<usize>,
    pub normal_id: Option<usize>,
    pub specular_id: Option<usize>,
}


impl MaterialTextureIds {
    // Returns an empty MaterialTextureIds with no textures assigned.
    pub const fn empty() -> Self {
        Self {
            diffuse_id: None,
            normal_id: None,
            specular_id: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub diffuse_color: Vec3A,   // Base color of the material
    pub albedo: [f32; 4],       // Albedo factors for diffuse, specular, reflection, and refraction
    pub specular_exponent: f32, // Also known as "shininess", controls the size of specular highlights
    pub refractive_index: f32,  // Index of refraction for transparent materials
    pub textures: MaterialTextureIds, // IDs of the textures used by this material
}

/*
    This struct represents a light source in the scene.
    It contains the position, intensity, and color of the light.
*/
#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub position: Vec3A, // Position of the light in world space
    pub intensity: f32, // Intensity of the light, affecting how bright it appears
    pub color: Vec3A, // Color of the light, allowing for colored lighting effects
}

impl Material {
    pub const fn empty() -> Self {
        Self {
            diffuse_color: Vec3A::ZERO,
            albedo: [0.0; 4],
            specular_exponent: 0.0,
            refractive_index: 1.0,
            textures: MaterialTextureIds::empty(),
        }
    }
}