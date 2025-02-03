use crate::prelude::*;

use super::{
    sprite::Sprite,
    utils::{CycleCounter, Timer},
};

pub struct AnimatedSprite {
    pub base: Sprite,
    pub frames: CycleCounter<u16>,

    offset: Vector2,
    timer: Timer,
}

impl AnimatedSprite {
    pub fn new(sprite: Sprite, max_frames: u16, speed: f32) -> Self {
        let offset = sprite.get_offset();
        Self {
            base: sprite,
            offset,
            frames: CycleCounter::new(0, max_frames),
            timer: Timer::new(0.0, speed),
        }
    }

    #[inline]
    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.timer.max = speed;
        self
    }

    #[inline]
    pub fn speed(&self) -> f32 {
        self.timer.max
    }

    #[inline]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.base.set_position(x, y);
        self
    }

    pub fn update(&mut self, delta: f32) {
        if !self.timer.tick(delta) {
            return;
        }

        let frame = self.frames.next();
        let x = self.base.width() * frame as f32;
        let y = self.base.get_offset_y();
        self.base.set_offset(x, y);
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        self.base.draw(d);
    }
}
