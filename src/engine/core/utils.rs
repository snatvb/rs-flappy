use num::{Num, One};
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
