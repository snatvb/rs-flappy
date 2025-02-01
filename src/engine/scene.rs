use super::{renderer::RendererHandler, Engine};

pub trait Scene {
    fn name(&self) -> &str;
    fn load(&mut self, _engine: &Engine) {}
    fn update(&mut self, _engine: &Engine) {}
    fn draw(&mut self, _engine: &Engine, _renderer: &mut RendererHandler) {}
    fn unload(&mut self, _engine: &Engine) {}
}
