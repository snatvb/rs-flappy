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
}
