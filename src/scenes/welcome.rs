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

    fn update(&mut self, engine: &Engine) {
        if engine.rl.borrow().is_key_pressed(KeyboardKey::KEY_SPACE)
            || engine
                .rl
                .borrow()
                .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            engine.switch_scene("game");
        }
    }

    fn draw(&mut self, _: &Engine, renderer: &mut RendererHandler) {
        const FZ: i32 = 10;
        renderer.draw(|d, r| {
            const TEXT: &str = "Press [Space] to start";
            let x = r.width as i32 / 2 - d.measure_text(TEXT, FZ) / 2;
            let y = r.height as i32 / 2 - FZ / 2;
            d.draw_text(TEXT, x, y, FZ, Color::WHITE);
        });
    }
}
