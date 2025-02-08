use std::rc::Rc;

use crate::prelude::*;

use crate::engine::{
    core::{animtaed_sprite::AnimatedSprite, sprite::Sprite},
    Engine,
};

const MAX_VELOCITY: f32 = 10.0;

pub struct Player {
    pub velocity: f32,
    sprite: AnimatedSprite,
    pub collider: Rectangle,
}

impl Player {
    pub fn new(texture: Rc<Texture2D>) -> Self {
        Self {
            velocity: Default::default(),
            sprite: AnimatedSprite::new(Sprite::new(texture, 16.0, 16.0), 4, 0.2),
            collider: Rectangle {
                width: 12.0,
                height: 12.0,
                x: 0.0,
                y: 0.0,
            },
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

    pub fn sync_collider(&mut self) {
        self.collider.x = self.sprite.x() + 2.0;
        self.collider.y = self.sprite.y() + 2.0;
    }

    pub fn jump(&mut self) {
        self.velocity = -2.0;
    }

    pub fn update(&mut self, engine: &Engine) {
        let dt = engine.delta.get();
        self.velocity = lerp(self.velocity, MAX_VELOCITY, 0.8_f32.min(dt * 0.4));
        self.sprite.update(dt);
        self.set_position(self.x(), self.y() + self.velocity);
    }

    #[inline]
    pub fn draw_gizmoz(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        d.draw_rectangle_lines(
            self.collider.x as i32,
            self.collider.y as i32,
            self.collider.width as i32,
            self.collider.height as i32,
            Color::GREEN,
        );
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.sprite.draw(d);
    }
}
