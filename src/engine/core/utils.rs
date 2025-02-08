use num::{Num, One};
use raylib::{color::Color, math::lerp};
use std::{
    ops::{Add, Rem},
    thread::current,
};

#[derive(Clone, Debug, Default)]
pub struct CycleCounter<T: Copy + Clone + Num> {
    pub current: T,
    pub max: T,
}

impl<T> CycleCounter<T>
where
    T: Copy + One + Num,
{
    pub fn new(inital: T, max: T) -> Self {
        Self {
            current: inital,
            max,
        }
    }

    pub fn next(&mut self) -> T {
        self.current = (self.current + T::one()) % self.max;
        self.current
    }
}

#[derive(Clone, Debug, Default)]
pub struct Timer {
    pub current: f32,
    pub max: f32,
}

impl Timer {
    pub fn new(current: f32, max: f32) -> Self {
        Self { current, max }
    }

    pub fn tick(&mut self, delta: f32) -> bool {
        self.current += delta;
        if self.current >= self.max {
            self.current = 0.0;
            return true;
        }

        false
    }
}

pub fn color_lerp(from: Color, to: Color, factor: f32) -> Color {
    let factor = factor.clamp(0.0, 1.0);
    Color {
        r: lerp(from.r as f32, to.r as f32, factor) as u8,
        g: lerp(from.g as f32, to.g as f32, factor) as u8,
        b: lerp(from.b as f32, to.b as f32, factor) as u8,
        a: lerp(from.a as f32, to.a as f32, factor) as u8,
    }
}
