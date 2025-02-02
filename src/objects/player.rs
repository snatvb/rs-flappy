use std::rc::Rc;

use raylib::prelude::*;

use crate::engine::{
    core::{animtaed_sprite::AnimatedSprite, sprite::Sprite},
    Engine,
};

pub struct Player {
    sprite: AnimatedSprite,
}

impl Player {
    pub fn new(texture: Rc<Texture2D>) -> Self {
        Self {
            sprite: AnimatedSprite::new(Sprite::new(texture, 16.0, 16.0), 4, 0.2),
        }
    }

    pub fn update(&mut self, engine: &Engine) {
        self.sprite.update(engine.delta.get());
    }

    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.sprite.draw(d);
    }
}
