use std::rc::Rc;

use crate::prelude::*;

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

    delegate! {
        to self.sprite {
            pub fn set_position(&mut self, x: f32, y: f32);
            pub fn x(&self) -> f32;
            pub fn y(&self) -> f32;
            pub fn width(&self) -> f32;
            pub fn height(&self) -> f32;
        }
    }

    pub fn update(&mut self, engine: &Engine) {
        self.sprite.update(engine.delta.get());
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.sprite.draw(d);
    }
}
