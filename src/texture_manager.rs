use raylib::prelude::*;

pub struct TextureManager {
    wall_1: Image,
    wall_2: Image,
    wall_3: Image,
    wall_4: Image,
    arrow: Image,
    floors: [Image; 2],
    ceilings: [Image; 2],
    portal_frames: [Image; 5],
    enemy_frames: [Image; 5],
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            wall_1: Image::load_image("assets/brick_wall.png")
                .expect("Failed to load oak log texture"),
            wall_2: Image::load_image("assets/cobblestone_wall.png")
                .expect("Failed to load cobblestone texture"),
            wall_3: Image::load_image("assets/nether_wall.png")
                .expect("Failed to load stone brick texture"),
            wall_4: Image::load_image("assets/quartz_wall.png")
                .expect("Failed to load stone wall texture"),
            arrow: Image::load_image("assets/arrow.png")
                .expect("Failed to load arrow texture"),
            floors: [
                Image::load_image("assets/planks_wall.png")
                    .expect("Failed to load world 1 floor"),
                Image::load_image("assets/nether_wall.png")
                    .expect("Failed to load world 2 floor"),
            ],
            ceilings: [
                Image::load_image("assets/brick_wall.png")
                    .expect("Failed to load world 1 ceiling"),
                Image::load_image("assets/quartz_wall.png")
                    .expect("Failed to load world 2 ceiling"),
            ],
            portal_frames: [
                Image::load_image("assets/nether_portal/portal_1.png").expect("Failed to load portal frame 1"),
                Image::load_image("assets/nether_portal/portal_2.png").expect("Failed to load portal frame 2"),
                Image::load_image("assets/nether_portal/portal_3.png").expect("Failed to load portal frame 3"),
                Image::load_image("assets/nether_portal/portal_4.png").expect("Failed to load portal frame 4"),
                Image::load_image("assets/nether_portal/portal_5.png").expect("Failed to load portal frame 5"),
            ],
            enemy_frames: [
                Image::load_image("assets/zombie/zombie_1.png").expect("Failed to load enemy frame 1"),
                Image::load_image("assets/zombie/zombie_2.png").expect("Failed to load enemy frame 2"),
                Image::load_image("assets/zombie/zombie_3.png").expect("Failed to load enemy frame 3"),
                Image::load_image("assets/zombie/zombie_4.png").expect("Failed to load enemy frame 4"),
                Image::load_image("assets/zombie/zombie_5.png").expect("Failed to load enemy frame 5"),
            ],

        }
    }

    pub fn get_pixel_color(&self, ch: char, u: f32, v: f32) -> Color {
        let image = match ch {
            '+' => &self.wall_1,
            '-' | '|' | 'g'   => &self.wall_2,
            '#' => &self.wall_3,
            '&' => &self.wall_4,
            _ => return Color::WHITE,
        };
        sample(image, u, v)
    }

    pub fn get_sprite_pixel(&self, ch: char, frame: usize, u: f32, v: f32) -> Color {
        let image = match ch {
            'e' if frame % 5 == 0 => &self.enemy_frames[0],
            'e' => &self.enemy_frames[frame % 5],
            'w' => &self.portal_frames[frame % 5],
            'a' => &self.arrow,
            _ => return Color::BLANK,
        };
        sample(image, u, v)
    }

    pub fn get_floor_pixel(&self, world: usize, u: f32, v: f32) -> Color {
        sample(&self.floors[world], u, v)
    }

    pub fn get_ceiling_pixel(&self, world: usize, u: f32, v: f32) -> Color {
        sample(&self.ceilings[world], u, v)
    }
}

fn sample(image: &Image, u: f32, v: f32) -> Color {
        let x = ((u.clamp(0.0, 1.0) * image.width() as f32) as i32).min(image.width() - 1);
        let y = ((v.clamp(0.0, 1.0) * image.height() as f32) as i32).min(image.height() - 1);
        image.get_color(x, y)
}
