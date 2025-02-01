use super::{renderer::RendererHandler, Engine};

pub trait Scene {
    fn name(&self) -> &str;
    fn load(&mut self, engine: &Engine) {}
    fn update(&mut self, engine: &Engine) {}
    fn draw(&mut self, engine: &Engine, renderer: &mut RendererHandler) {}
    fn unload(&mut self, engine: &Engine) {}
}
