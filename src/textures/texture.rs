use glam::Vec3A;
/*
    This class represents a texture loaded from an image file (PNG/JPG) 
    and provides functionality to sample colors based on UV coordinates. 
    It supports both sRGB and linear color spaces, allowing for proper 
    handling of different types of textures such as diffuse maps 
    and normal/specular maps.
*/

#[derive(Clone, Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec3A>,
}

impl Texture {
    // Empty texture constructor for cases where no texture is provided
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            width: 0,
            height: 0,
            pixels: Vec::new(),
        }
    }

    /// Load a texture from an image file (PNG/JPG)
    /// `is_srgb = true` for Diffuse (linearizes with pow 2.2)
    /// `is_srgb = false` for Normal and Specular (direct vector data)
    pub fn load(path: &str, is_srgb: bool) -> Self {
        let img = image::open(path)
            .unwrap_or_else(|_| panic!("No se pudo cargar la imagen: {}", path))
            .into_rgb8();

        // Get the dimensions of the image
        let width = img.width();
        let height = img.height();

        // Convert the image pixels to Vec3A format, applying sRGB to linear conversion if needed
        // Map each pixel to a Vec3A, normalizing the RGB values to [0, 1] range
        let pixels = img
            .pixels()
            .map(|p| {
                let mut r = p.0[0] as f32 / 255.0;
                let mut g = p.0[1] as f32 / 255.0;
                let mut b = p.0[2] as f32 / 255.0;

                // If the texture is in sRGB space, convert it to linear 
                // space using the inverse gamma correction (pow 2.2)
                if is_srgb {
                    r = r.powf(2.2);
                    g = g.powf(2.2);
                    b = b.powf(2.2);
                }

                // Return the color as a Vec3A
                Vec3A::new(r, g, b)
            })
            .collect();

        Self { width, height, pixels }
    }

    // Sample the texture color at given UV coordinates (u, v)
    #[inline(always)]
    pub fn sample(&self, u: f32, v: f32) -> Vec3A {
        if self.pixels.is_empty() {
            return Vec3A::ONE;
        }

        let u_wrap = u.fract().abs();
        let v_wrap = v.fract().abs();

        let x = ((u_wrap * self.width as f32) as u32).min(self.width - 1);
        let y = ((v_wrap * self.height as f32) as u32).min(self.height - 1);

        let index = (y * self.width + x) as usize;
        self.pixels[index]
    }
}