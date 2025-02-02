use raylib::prelude::*;

pub struct Sprite {
    pub texture: Texture2D,
    pub rotation: f32,
    pub position: Vector2,

    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
}

impl Sprite {
    pub fn new(texture: Texture2D, w: f32, h: f32) -> Self {
        Self {
            texture,
            rotation: 0.0,
            position: Default::default(),
            source: Rectangle::new(0.0, 0.0, w, h),
            dest: Rectangle::new(0.0, 0.0, w, h),
            origin: Default::default(),
        }
    }

    pub fn set_offset(&mut self, x: f32, y: f32) -> &mut Self {
        self.source.x = x;
        self.source.y = y;
        self
    }

    pub fn get_offset(&self) -> Vector2 {
        Vector2::new(self.source.x, self.source.y)
    }

    pub fn resize(&mut self, w: f32, h: f32) -> &mut Self {
        self.dest.width = w;
        self.dest.height = h;
        self
    }

    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.position.x = x;
        self.position.y = y;
        self
    }

    pub fn width(&self) -> f32 {
        self.dest.width
    }

    pub fn height(&self) -> f32 {
        self.dest.height
    }

    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        d.draw_texture_pro(
            &self.texture,
            self.source,
            self.dest,
            Vector2::default(),
            0f32,
            Color::WHITE,
        )
    }
}
