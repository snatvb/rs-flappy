use raylib::prelude::*;

use crate::engine::{renderer::RendererHandler, scene::Scene, Engine};

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for Game {
    fn name(&self) -> &str {
        "game"
    }

    fn draw(&mut self, _: &Engine, renderer: &mut RendererHandler) {
        const FZ: i32 = 10;
        renderer.draw(|d, r| {
            const TEXT: &str = "Press [Space] to jump";
            let x = r.width as i32 / 2 - d.measure_text(TEXT, FZ) / 2;
            let y = r.height as i32 / 2 - FZ / 2;
            d.draw_text(TEXT, x, y, FZ, Color::WHITE);
        });
    }
}
