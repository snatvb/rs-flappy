use raylib::prelude::*;

use crate::engine::core::sprite::Sprite;

pub struct Player {
    sprite: Sprite,
}

impl Player {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            sprite: Sprite::new(texture, 16.0, 16.0),
        }
    }

    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.sprite.draw(d);
    }
}
