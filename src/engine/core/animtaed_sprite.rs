use raylib::prelude::*;

use super::{
    sprite::Sprite,
    utils::{CycleCounter, Timer},
};

pub struct AnimatedSprite {
    pub sprite: Sprite,
    pub frames: CycleCounter<u16>,

    offset: Vector2,
    timer: Timer,
}

impl AnimatedSprite {
    pub fn new(sprite: Sprite, max_frames: u16, speed: f32) -> Self {
        let offset = sprite.get_offset();
        Self {
            sprite,
            offset,
            frames: CycleCounter::new(0, max_frames),
            timer: Timer::new(0.0, speed),
        }
    }

    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.timer.max = speed;
        self
    }

    pub fn speed(&self) -> f32 {
        self.timer.max
    }

    pub fn update(&mut self, delta: f32) {
        if !self.timer.tick(delta) {
            return;
        }

        let frame = self.frames.next();
        let x = self.sprite.width() * frame as f32;
        let y = self.sprite.get_offset_y();
        self.sprite.set_offset(x, y);
    }

    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.sprite.draw(d);
    }
}
