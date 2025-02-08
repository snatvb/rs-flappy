use super::{renderer::RendererHandler, Engine};

pub trait SceneMessage {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Scene {
    fn name(&self) -> &str;
    fn preload_message(&mut self, _engine: &Engine, _msg: Box<dyn SceneMessage>) {}
    fn load(&mut self, _engine: &Engine) {}
    fn update(&mut self, _engine: &Engine) {}
    fn draw(&mut self, _engine: &Engine, _renderer: &mut RendererHandler) {}
    fn unload(&mut self, _engine: &Engine) {}
}
