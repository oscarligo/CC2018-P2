use raylib::prelude::*;
use rayon::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
    background_color: Color,
    current_color: Color,
    texture: Texture2D,
}

impl Framebuffer {
    pub fn new(
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        width: u32,
        height: u32,
        background_color: Color,
    ) -> Self {
        let total_pixels = (width * height) as usize;
        let pixels = vec![background_color; total_pixels];
        let img = Image::gen_image_color(width as i32, height as i32, background_color);
        let texture = window
            .load_texture_from_image(raylib_thread, &img)
            .expect("No se pudo inicializar la textura en GPU");

        Framebuffer {
            width,
            height,
            pixels,
            background_color,
            current_color: Color::WHITE,
            texture,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.pixels.fill(self.background_color);
    }

    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = self.current_color;
        }
    }

    #[inline(always)]
    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear();
    }

    #[inline(always)]
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize]
        } else {
            self.background_color
        }
    }

    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                self.pixels.len() * std::mem::size_of::<Color>(),
            )
        };

        self.texture.update_texture(bytes);

        let mut renderer = window.begin_drawing(raylib_thread);
        renderer.clear_background(Color::BLACK);
        renderer.draw_texture(&self.texture, 0, 0, Color::WHITE);
    }

    pub fn render_parallel<F>(&mut self, render_pixel: F)
    where
        F: Fn(u32, u32) -> Color + Sync + Send,
    {
        let width = self.width;

        self.pixels
            .par_chunks_exact_mut(width as usize)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, pixel) in row.iter_mut().enumerate() {
                    *pixel = render_pixel(x as u32, y as u32);
                }
            });
    }
}