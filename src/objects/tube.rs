use derive_more::Display;

use crate::prelude::*;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum Pos {
    Top,
    Bottom,
}

pub struct Tube {
    pub sprite: Sprite,
    pub variant: u8,
    pub pos: Pos,
}

impl Tube {
    delegate! {
        to self.sprite {
            pub fn set_position(&mut self, x: f32, y: f32);
            pub fn x(&self) -> f32;
            pub fn y(&self) -> f32;
            pub fn width(&self) -> f32;
            pub fn height(&self) -> f32;
        }
    }

    #[inline]
    pub fn shift(&mut self, x: f32, y: f32) {
        self.set_position(self.x() + x, self.y() + y);
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.sprite.draw(d);
    }
}
