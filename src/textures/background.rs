use crate::vector::Vec3A;
use std::f32::consts::PI;
/*
This module provides functionality for loading and sampling background textures,
supporting both PNG and HDR formats. It includes gamma correction for PNG images and a
llows for efficient sampling based on 3D direction vectors.
*/


const INV_TWO_PI: f32 = 1.0 / (2.0 * PI); // Precomputed constant for efficiency in texture sampling
const INV_PI: f32 = 1.0 / PI; // Precomputed constant for efficiency in texture sampling

pub struct BackgroundTexture {
    pub width: u32,
    pub height: u32,
    width_f32: f32,
    height_f32: f32,
    pub pixels: Vec<Vec3A>,
}

impl BackgroundTexture {
    /// Loads a PNG image from the specified path, applies gamma correction, and stores the pixel data in a vector of Vec3A.
    pub fn load_png(path: &str) -> Self {
        let dynamic_img = image::open(path).expect("No se pudo abrir la imagen PNG");
        let rgb_img = dynamic_img.into_rgb8();

        let width = rgb_img.width();
        let height = rgb_img.height();

        let mut gamma_lut = [0.0f32; 256];
        for (i, val) in gamma_lut.iter_mut().enumerate() {
            *val = (i as f32 / 255.0).powf(2.2);
        }

        let raw_bytes = rgb_img.as_raw();
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for chunk in raw_bytes.chunks_exact(3) {
            let r = gamma_lut[chunk[0] as usize];
            let g = gamma_lut[chunk[1] as usize];
            let b = gamma_lut[chunk[2] as usize];
            pixels.push(Vec3A::new(r, g, b));
        }

        Self {
            width,
            height,
            width_f32: width as f32,
            height_f32: height as f32,
            pixels,
        }
    }

    
    pub fn load_hdr(path: &str) -> Self {
        let dynamic_img = image::open(path).expect("No se pudo abrir la imagen HDR");
        let rgb_img = dynamic_img.into_rgb32f();

        let width = rgb_img.width();
        let height = rgb_img.height();

        let raw_floats = rgb_img.as_raw();
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for chunk in raw_floats.chunks_exact(3) {
            pixels.push(Vec3A::new(chunk[0], chunk[1], chunk[2]));
        }

        Self {
            width,
            height,
            width_f32: width as f32,
            height_f32: height as f32,
            pixels,
        }
    }

    pub fn load(path: &str) -> Self {
        if path.ends_with(".hdr") {
            Self::load_hdr(path)
        } else {
            Self::load_png(path)
        }
    }

    #[inline(always)]
    pub fn sample(&self, dir: Vec3A) -> Vec3A {
        let u = 0.5 + dir.z.atan2(dir.x) * INV_TWO_PI;
        let v = 0.5 - dir.y.clamp(-1.0, 1.0).asin() * INV_PI;

        let x = ((u * self.width_f32) as u32).min(self.width - 1);
        let y = ((v * self.height_f32) as u32).min(self.height - 1);

        let index = (y * self.width + x) as usize;

        unsafe { *self.pixels.get_unchecked(index) }
    }
}
