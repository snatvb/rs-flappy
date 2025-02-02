use raylib::prelude::*;

use crate::engine::renderer::RendererHandler;
use crate::engine::scene::Scene;
use crate::engine::Engine;
use crate::objects::player::Player;

pub struct Game {
    player: Option<Player>,
}

impl Game {
    pub fn new() -> Self {
        Self { player: None }
    }
}

impl Scene for Game {
    fn name(&self) -> &str {
        "game"
    }

    fn load(&mut self, _engine: &Engine) {
        let texture = _engine
            .rl
            .borrow_mut()
            .load_texture(&_engine.thread, "assets/birds.png")
            .expect("Birds png must be defined");
        log::debug!("Texture loaded");

        self.player = Some(Player::new(texture));
    }

    fn draw(&mut self, _: &Engine, renderer: &mut RendererHandler) {
        const FZ: i32 = 10;
        renderer.draw(|d, r| {
            let player = self
                .player
                .as_ref()
                .expect("Player must be initialized to draw the game");
            const TEXT: &str = "Press [Space] to jump";
            let x = r.width as i32 / 2 - d.measure_text(TEXT, FZ) / 2;
            let y = r.height as i32 / 2 - FZ / 2;
            d.draw_text(TEXT, x, y, FZ, Color::WHITE);
            player.draw(d);
        });
    }
}
