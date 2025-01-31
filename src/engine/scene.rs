use super::Engine;

pub trait Scene {
    fn name(&self) -> &str;
    fn load(&mut self, engine: &Engine);
    fn update(&mut self, delta: f32);
    fn draw(&mut self, delta: f32);
    fn unload(&mut self);
}
