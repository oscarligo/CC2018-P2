use raylib::prelude::*;
use std::num::NonZeroUsize;
use std::thread;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
    texture: Texture2D,
}

pub struct TitleScreen<'a> {
    pub title: &'a Texture2D,
    pub singleplayer: &'a Texture2D,
    pub quit: &'a Texture2D,
    pub singleplayer_bounds: Rectangle,
    pub quit_bounds: Rectangle,
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
        toast: Option<(&Texture2D, f32)>,
        hotbar: &Texture2D,
        hotbar_selector: &Texture2D,
        block_icons: &[Texture2D],
        selected_block: usize,
        effect: &Texture2D,
        effect_selector: &Texture2D,
        hearts: &[Texture2D],
        selected_effect: usize,
        blur_radius: u32,
        title_screen: Option<TitleScreen<'_>>,
        show_hud: bool,
    ) {
        if pixels_changed {
            let blurred = (blur_radius > 0)
                .then(|| blur_pixels(&self.pixels, self.width, self.height, blur_radius));
            let pixels = blurred.as_deref().unwrap_or(&self.pixels);
            let total_bytes = pixels.len() * std::mem::size_of::<Color>();
            let raw_pixel_slice =
                unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const u8, total_bytes) };
            let _ = self.texture.update_texture(raw_pixel_slice);
        }

        let mut renderer = window.begin_drawing(raylib_thread);
        renderer.clear_background(Color::BLACK);
        renderer.draw_texture(&self.texture, 0, 0, Color::WHITE);

        if let Some(title_screen) = title_screen {
            let title_width = self.width as f32 * 0.62;
            let title_height =
                title_width * title_screen.title.height as f32 / title_screen.title.width as f32;
            renderer.draw_texture_pro(
                title_screen.title,
                Rectangle::new(
                    0.0,
                    0.0,
                    title_screen.title.width as f32,
                    title_screen.title.height as f32,
                ),
                Rectangle::new(
                    (self.width as f32 - title_width) * 0.5,
                    -title_height * 0.18,
                    title_width,
                    title_height,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
            for (texture, bounds) in [
                (title_screen.singleplayer, title_screen.singleplayer_bounds),
                (title_screen.quit, title_screen.quit_bounds),
            ] {
                renderer.draw_texture_pro(
                    texture,
                    Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
                    bounds,
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
        }

        if show_hud && let Some((toast, progress)) = toast {
            let target_width = (self.width as f32 * 0.36).min(360.0);
            let scale = target_width / toast.width as f32;
            let progress = progress.clamp(0.0, 1.0);
            let progress = progress * progress * (3.0 - 2.0 * progress);
            let hidden_x = self.width as f32;
            let visible_x = self.width as f32 - target_width - 24.0;
            renderer.draw_texture_ex(
                toast,
                Vector2::new(hidden_x + (visible_x - hidden_x) * progress, 24.0),
                0.0,
                scale,
                Color::WHITE,
            );
        }

        if !show_hud {
            return;
        }

        let hotbar_x = (self.width as f32 - hotbar.width as f32) * 0.5;
        let hotbar_y = self.height as f32 - hotbar.height as f32 - 20.0;
        renderer.draw_texture_v(hotbar, Vector2::new(hotbar_x, hotbar_y), Color::WHITE);
        for (slot, icon) in block_icons.iter().enumerate() {
            renderer.draw_texture_pro(
                icon,
                Rectangle::new(0.0, 0.0, icon.width as f32, icon.height as f32),
                Rectangle::new(
                    hotbar_x + 6.0 + slot as f32 * 40.0,
                    hotbar_y + 6.0,
                    32.0,
                    32.0,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
        renderer.draw_texture_v(
            hotbar_selector,
            Vector2::new(
                hotbar_x - 2.0 + selected_block as f32 * 40.0,
                hotbar_y - 2.0,
            ),
            Color::WHITE,
        );

        let effect_scale = 2.0;
        let effect_size = effect.width as f32 * effect_scale;
        let effect_step = effect_size + 4.0;
        for slot in 0..hearts.len() {
            let x = 20.0 + slot as f32 * effect_step;
            renderer.draw_texture_ex(
                effect,
                Vector2::new(x, 20.0),
                0.0,
                effect_scale,
                Color::WHITE,
            );
        }
        renderer.draw_texture_ex(
            effect_selector,
            Vector2::new(20.0 + selected_effect as f32 * effect_step, 20.0),
            0.0,
            effect_scale,
            Color::WHITE,
        );
        for (slot, heart) in hearts.iter().enumerate() {
            let x = 20.0 + slot as f32 * effect_step;
            let heart_size = 22.0;
            renderer.draw_texture_pro(
                heart,
                Rectangle::new(0.0, 0.0, heart.width as f32, heart.height as f32),
                Rectangle::new(
                    x + (effect_size - heart_size) * 0.5,
                    20.0 + (effect_size - heart_size) * 0.5,
                    heart_size,
                    heart_size,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
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

fn blur_pixels(pixels: &[Color], width: u32, height: u32, radius: u32) -> Vec<Color> {
    let width = width as usize;
    let height = height as usize;
    let radius = radius as isize;
    let offsets = [-radius, 0, radius];
    let mut blurred = Vec::with_capacity(pixels.len());

    for y in 0..height {
        for x in 0..width {
            let mut red = 0u32;
            let mut green = 0u32;
            let mut blue = 0u32;
            for offset_y in offsets {
                for offset_x in offsets {
                    let sample_x = (x as isize + offset_x).clamp(0, width as isize - 1) as usize;
                    let sample_y = (y as isize + offset_y).clamp(0, height as isize - 1) as usize;
                    let sample = pixels[sample_y * width + sample_x];
                    red += sample.r as u32;
                    green += sample.g as u32;
                    blue += sample.b as u32;
                }
            }
            blurred.push(Color::new(
                (red / 9) as u8,
                (green / 9) as u8,
                (blue / 9) as u8,
                255,
            ));
        }
    }
    blurred
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
