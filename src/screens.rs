use raylib::prelude::*;


pub fn draw_screen(
    window: &mut RaylibHandle,
    thread: &RaylibThread,
    texture: &Texture2D,
    width: i32,
    height: i32,
) {
    let mut renderer = window.begin_drawing(thread);
    renderer.clear_background(Color::BLACK);
    renderer.draw_texture_pro(
        texture,
        Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
        Rectangle::new(0.0, 0.0, width as f32, height as f32),
        Vector2::zero(),
        0.0,
        Color::WHITE,
    );
}

pub fn next_world(selected: usize) -> usize {
    selected ^ 1
}