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
    #[inline]
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.sprite.set_position(x, y);
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.sprite.x()
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.sprite.y()
    }

    #[inline]
    pub fn shift(&mut self, x: f32, y: f32) {
        let position = self.sprite.get_position();
        self.set_position(position.x + x, position.y + y);
        // log::debug!("Move: {position:?} -> +{x}, +{y}");
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.sprite.draw(d);
    }
}
