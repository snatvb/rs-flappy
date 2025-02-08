use crate::prelude::*;

pub struct Sprite {
    pub texture: Asset2D,
    pub rotation: f32,

    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
}

pub enum Direction {
    Left,
    Right,
    Up,
    UpSide,
}

impl Sprite {
    pub fn new(texture: Asset2D, w: f32, h: f32) -> Self {
        Self {
            texture,
            rotation: 0.0,
            source: Rectangle::new(0.0, 0.0, w, h),
            dest: Rectangle::new(0.0, 0.0, w, h),
            origin: Default::default(),
        }
    }

    #[inline]
    pub fn display_rect(&self) -> &Rectangle {
        &self.dest
    }

    #[inline]
    pub fn set_offset(&mut self, x: f32, y: f32) -> &mut Self {
        self.source.x = x;
        self.source.y = y;
        self
    }

    #[inline]
    pub fn get_offset(&self) -> Vector2 {
        Vector2::new(self.source.x, self.source.y)
    }

    #[inline]
    pub fn get_offset_x(&self) -> f32 {
        self.source.x
    }

    #[inline]
    pub fn get_offset_y(&self) -> f32 {
        self.source.y
    }

    #[inline]
    pub fn resize(&mut self, w: f32, h: f32) {
        self.dest.width = w;
        self.dest.height = h;
    }

    #[inline]
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.dest.x = x;
        self.dest.y = y;
    }

    #[inline]
    pub fn get_position(&self) -> Vector2 {
        Vector2 {
            x: self.dest.x,
            y: self.dest.y,
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.dest.x
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.dest.y
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.dest.width
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.dest.height
    }

    #[inline]
    pub fn flip_x(&mut self) {
        self.source.width *= -1.0;
    }

    #[inline]
    pub fn flip_y(&mut self) {
        self.source.height *= -1.0;
    }

    #[inline]
    pub fn flip(&mut self, to: Direction) {
        match (self.width(), self.height(), to) {
            (w, _, Direction::Right) if w < 0.0 => self.flip_x(),
            (w, _, Direction::Left) if w > 0.0 => self.flip_x(),
            (_, h, Direction::UpSide) if h > 0.0 => self.flip_y(),
            (_, h, Direction::Up) if h < 0.0 => self.flip_y(),
            _ => {}
        }
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        d.draw_texture_pro(
            self.texture.as_ref(),
            self.source,
            self.dest,
            self.origin,
            0f32,
            Color::WHITE,
        )
    }
}
