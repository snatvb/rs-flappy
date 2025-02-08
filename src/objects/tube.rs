use derive_more::Display;

use crate::prelude::*;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum Pos {
    Top,
    Bottom,
}

pub struct Tube {
    pub sprite: Sprite,
    pub pos: Pos,
    pub visited: bool,
}

impl Tube {
    delegate! {
        to self.sprite {
            pub fn set_position(&mut self, x: f32, y: f32);
            pub fn x(&self) -> f32;
            pub fn y(&self) -> f32;
            pub fn width(&self) -> f32;
            pub fn height(&self) -> f32;
            pub fn flip_x(&mut self);
            pub fn flip_y(&mut self);
            pub fn flip(&mut self, to: sprite::Direction);
        }
    }

    pub fn set_variant(&mut self, variant: u8) {
        self.sprite.set_offset(
            variant as f32 * self.sprite.width(),
            self.sprite.get_offset_y(),
        );
    }

    #[inline]
    pub fn shift(&mut self, x: f32, y: f32) {
        self.set_position(self.x() + x, self.y() + y);
    }

    #[inline]
    pub fn draw_gizmoz(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        d.draw_rectangle_lines(
            self.sprite.display_rect().x as i32,
            self.sprite.display_rect().y as i32,
            self.sprite.display_rect().width as i32,
            self.sprite.display_rect().height as i32,
            Color::GREEN,
        );
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.sprite.draw(d);
    }
}
