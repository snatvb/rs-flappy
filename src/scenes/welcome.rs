use raylib::prelude::*;

use crate::engine::{renderer::RendererHandler, scene::Scene, Engine};

pub struct Welcome {}

impl Welcome {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for Welcome {
    fn name(&self) -> &str {
        "welcome"
    }

    fn draw(&mut self, _: &Engine, renderer: &mut RendererHandler) {
        const FZ: i32 = 10;
        renderer.draw(|d, r| {
            let text = "Press [Space] to start";
            let x = r.width as i32 / 2 - d.measure_text(text, FZ) / 2;
            let y = r.height as i32 / 2 - FZ / 2;
            d.draw_text(text, x, y, FZ, Color::WHITE);
        });
    }
}
