use glam::Vec3A;
use std::fs::File;
use std::io::BufReader;

pub struct BackgroundTexture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec3A>,
}

impl BackgroundTexture {
    pub fn load_hdr(path: &str) -> Self {
        let dynamic_img = image::open(path).expect("No se pudo abrir la imagen HDR");
        let rgb_img = dynamic_img.into_rgb32f();

        let width = rgb_img.width();
        let height = rgb_img.height();

        let pixels = rgb_img
            .pixels()
            .map(|p| Vec3A::new(p.0[0], p.0[1], p.0[2]))
            .collect();

        Self {
            width,
            height,
            pixels,
        }
    }

    #[inline(always)]
    pub fn sample(&self, dir: &Vec3A) -> Vec3A {
        // Coordenada horizontal (longitud/acimut): [-PI, PI] -> [0.0, 1.0]
        let u = 0.5 + dir.z.atan2(dir.x) / (2.0 * std::f32::consts::PI);

        let v = 0.5 - dir.y.clamp(-1.0, 1.0).asin() / std::f32::consts::PI;

        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);

        let index = (y * self.width + x) as usize;
        self.pixels[index]
    }
}