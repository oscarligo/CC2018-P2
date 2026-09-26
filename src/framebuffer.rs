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

    pub fn set_pixel_block(&mut self, x: u32, y: u32, size: u32, color: Color) {
        fill_block(
            &mut self.pixels,
            self.width as usize,
            self.height as usize,
            x as usize,
            y as usize,
            size.max(1) as usize,
            color,
        );
    }

    pub fn swap_buffers(
        &mut self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        pixels_changed: bool,
    ) {
        if pixels_changed {
            // `raylib::ffi::Color` garantiza 4 bytes contiguos en memoria (RGBA8888).
            let total_bytes = self.pixels.len() * std::mem::size_of::<Color>();
            let raw_pixel_slice = unsafe {
                std::slice::from_raw_parts(self.pixels.as_ptr() as *const u8, total_bytes)
            };
            let _ = self.texture.update_texture(raw_pixel_slice);
        }

        let mut renderer = window.begin_drawing(raylib_thread);
        renderer.clear_background(Color::BLACK);
        renderer.draw_texture(&self.texture, 0, 0, Color::WHITE);
        renderer.draw_fps(10, 10);
    }

    // Parallel rendering function that divides the framebuffer into chunks for each thread.
    pub fn render_parallel<F>(&mut self, pixel_size: u32, render_pixel: F)
    where
        F: Fn(u32, u32) -> Color + Sync + Send,
    {
        let width = self.width as usize;
        let height = self.height as usize;
        let pixel_size = pixel_size.max(1) as usize;

        if width == 0 || height == 0 {
            return;
        }

        let worker_count = thread::available_parallelism()
            .map(NonZeroUsize::get)
            .unwrap_or(1);

        // Calculates the number of rows each thread will process
        let sample_rows = height.div_ceil(pixel_size);
        let rows_per_worker = sample_rows.div_ceil(worker_count) * pixel_size;
        let chunk_pixel_count = rows_per_worker * width;
        let render_pixel = &render_pixel;

        thread::scope(|scope| {
            for (worker_id, row_chunk) in self.pixels.chunks_mut(chunk_pixel_count).enumerate() {
                let start_row = (worker_id * rows_per_worker) as u32;

                scope.spawn(move || {
                    let chunk_height = row_chunk.len() / width;
                    for local_y in (0..chunk_height).step_by(pixel_size) {
                        let sample_y =
                            (start_row as usize + local_y + pixel_size / 2).min(height - 1) as u32;

                        for x in (0..width).step_by(pixel_size) {
                            let sample_x = (x + pixel_size / 2).min(width - 1) as u32;
                            let color = render_pixel(sample_x, sample_y);
                            fill_block(
                                row_chunk,
                                width,
                                chunk_height,
                                x,
                                local_y,
                                pixel_size,
                                color,
                            );
                        }
                    }
                });
            }
        });
    }
}

// auxiliary function to fill a block of pixels in the framebuffer with a specific color
fn fill_block(
    pixels: &mut [Color],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    size: usize,
    color: Color,
) {
    let x_end = (x + size).min(width);
    let y_end = (y + size).min(height);

    for row in y..y_end {
        pixels[row * width + x..row * width + x_end].fill(color);
    }
}