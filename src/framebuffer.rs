use raylib::prelude::*;
use std::thread;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
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
            texture,
        }
    }

    #[inline(always)]
    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }

    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                self.pixels.len() * std::mem::size_of::<Color>(),
            )
        };

        let _ = self.texture.update_texture(bytes);

        let mut renderer = window.begin_drawing(raylib_thread);
        renderer.clear_background(Color::BLACK);
        renderer.draw_texture(&self.texture, 0, 0, Color::WHITE);
    }

    pub fn render_parallel<F>(&mut self, render_pixel: F)
    where
        F: Fn(u32, u32) -> Color + Sync + Send,
    {
        let width = self.width as usize;
        let height = self.height as usize;

        // Detects the number of available threads
        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        // Calculates the number of rows each thread will process
        let rows_per_thread = (height + num_threads - 1) / num_threads;
        let chunk_size = rows_per_thread * width;

        let render_pixel = &render_pixel;

        thread::scope(|s| {
            for (thread_idx, chunk) in self.pixels.chunks_mut(chunk_size).enumerate() {
                let start_row = thread_idx * rows_per_thread;

                s.spawn(move || {
                    for (row_offset, row) in chunk.chunks_exact_mut(width).enumerate() {
                        let y = (start_row + row_offset) as u32;
                        for (x, pixel) in row.iter_mut().enumerate() {
                            *pixel = render_pixel(x as u32, y);
                        }
                    }
                });
            }
        });
    }
}
