use raylib::prelude::*;
use std::num::NonZeroUsize;
use std::thread;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
    texture: Texture2D,
}

impl Framebuffer {
    // Constructor
    pub fn new(
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        width: u32,
        height: u32,
        background_color: Color,
    ) -> Self {
        let total_pixels = (width as usize) * (height as usize);
        let pixels = vec![background_color; total_pixels];

        let initial_image = Image::gen_image_color(width as i32, height as i32, background_color);
        let texture = window
            .load_texture_from_image(raylib_thread, &initial_image)
            .expect("Error al inicializar la textura en GPU");

        Self {
            width,
            height,
            pixels,
            texture,
        }
    }

    /// Paint a pixel in the framebuffer with the given color.
    #[inline(always)]
    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }

    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        // `raylib::ffi::Color` garantiza 4 bytes contiguos en memoria (RGBA8888).
        let total_bytes = self.pixels.len() * std::mem::size_of::<Color>();
        let raw_pixel_slice = unsafe {
            std::slice::from_raw_parts(self.pixels.as_ptr() as *const u8, total_bytes)
        };

        // Copia directa a la memoria de la textura en VRAM
        let _ = self.texture.update_texture(raw_pixel_slice);

        let mut renderer = window.begin_drawing(raylib_thread);
        renderer.clear_background(Color::BLACK);
        renderer.draw_texture(&self.texture, 0, 0, Color::WHITE);
        renderer.draw_fps(10, 10);
    }

    // Parallel rendering function that divides the framebuffer into chunks for each thread.
    pub fn render_parallel<F>(&mut self, render_pixel: F)
    where
        F: Fn(u32, u32) -> Color + Sync + Send,
    {
        let width = self.width as usize;
        let height = self.height as usize;

        if width == 0 || height == 0 {
            return;
        }

        let worker_count = thread::available_parallelism()
            .map(NonZeroUsize::get)
            .unwrap_or(1);

        // Calculates the number of rows each thread will process
        let rows_per_worker = (height + worker_count - 1) / worker_count;
        let chunk_pixel_count = rows_per_worker * width;
        let render_pixel = &render_pixel;

        thread::scope(|scope| {
            for (worker_id, row_chunk) in self.pixels.chunks_mut(chunk_pixel_count).enumerate() {
                let start_row = (worker_id * rows_per_worker) as u32;

                scope.spawn(move || {
                    for (row_offset, scanline) in row_chunk.chunks_exact_mut(width).enumerate() {
                        let current_y = start_row + row_offset as u32;

                        for (current_x, pixel) in scanline.iter_mut().enumerate() {
                            *pixel = render_pixel(current_x as u32, current_y);
                        }
                    }
                });
            }
        });
    }
}
