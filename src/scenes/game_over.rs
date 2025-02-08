use crate::{engine::scene::SceneMessage, prelude::*};

#[derive(Default)]
pub struct GameOver {
    score: u32,
}

pub struct Message {
    pub score: u32,
}

impl SceneMessage for Message {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl GameOver {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Scene for GameOver {
    fn name(&self) -> &str {
        "game_over"
    }

    fn preload_message(&mut self, _engine: &Engine, _msg: Box<dyn SceneMessage>) {
        if let Some(msg) = _msg.as_any().downcast_ref::<Message>() {
            self.score = msg.score;
        } else {
            log::error!("Unsupportaed message")
        }
    }

    fn update(&mut self, engine: &Engine) {
        if engine.rl.borrow().is_key_pressed(KeyboardKey::KEY_SPACE) {
            engine.switch_scene("game");
        }
    }

    fn draw(&mut self, _: &Engine, renderer: &mut RendererHandler) {
        const FZ: i32 = 10;
        const FZ_SCORE: i32 = 20;
        renderer.draw(|d, r| {
            const TEXT: &str = "Press [Space] to try again";
            let x = r.width as i32 / 2 - d.measure_text(TEXT, FZ) / 2;
            let y = r.height as i32 / 2 - FZ / 2;
            d.draw_text(TEXT, x, y, FZ, Color::WHITE);

            let text = &format!("Score earned: {}", self.score);
            let x = r.width as i32 / 2 - d.measure_text(text, FZ_SCORE) / 2;
            let y = y - FZ_SCORE - 4;
            d.draw_text(text, x, y, FZ_SCORE, Color::WHITE);
        });
    }
}
