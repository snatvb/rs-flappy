use std::borrow::BorrowMut;
use std::usize;

use rand::prelude::IndexedRandom;

use crate::engine::Engine;
use crate::objects::{tube, Ground, Player, Tube};
use crate::prelude::*;

struct Tubes {
    pool: Vec<Tube>,
    active: Vec<Tube>,
    texture: Asset2D,
}

const LAYERS: u32 = 4;

impl Tubes {
    pub fn new(texture: Asset2D) -> Self {
        Self {
            texture,
            pool: Default::default(),
            active: Default::default(),
        }
    }

    // TODO: Change variant
    // TODO: Add flip Sprite
    pub fn spawn(&mut self, enigne: &Engine, pos: tube::Pos, offset: u32) {
        let variant = 0;

        let (width, height) = enigne.renderer.borrow().size();
        let y = if pos == tube::Pos::Bottom {
            (height - 32 * (LAYERS - 1) + 24) - offset
        } else {
            16 - offset
        };

        let x = width + 32;

        let mut tube = self.pool.pop().unwrap_or_else(|| {
            log::debug!("Create new tube {pos} {variant}");

            Tube {
                sprite: Sprite::new(self.texture.clone(), 32.0, 48.0),
                variant,
                pos,
            }
        });

        tube.variant = variant;
        tube.pos = pos;
        tube.set_position(x as f32, y as f32);

        self.active.push(tube);
        {
            let amount = self.active.len();
            log::debug!("Spawned tube, active: {amount}");
        }
    }

    pub fn update(&mut self, engine: &Engine) {
        let mut to_remove: Vec<usize> = vec![];
        for (i, tube) in &mut self.active.iter_mut().enumerate() {
            tube.shift(-2.0, 0.0);
            if tube.x() < -32.0 {
                to_remove.push(i);
            }
        }
        for i in to_remove.iter().copied().rev() {
            let tube = self.active.remove(i);
            self.pool.push(tube);
        }

        // log::debug!("Active {}, Pool: {}", self.active.len(), self.pool.len());
    }

    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        for tube in &self.active {
            tube.draw(d);
        }
    }
}

struct State {
    player: Player,
    tubes: Tubes,
    ground: Ground,
}

pub struct Game {
    state: Option<State>,
}

impl Game {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl Scene for Game {
    fn name(&self) -> &str {
        "game"
    }

    fn load(&mut self, engine: &Engine) {
        let rl = &mut engine.rl.borrow_mut();
        let texture = engine
            .assets
            .load_texture(rl, &engine.thread, "birds.png")
            .expect("Birds png must be defined");

        let mut player = Player::new(texture);
        player.set_position(10.0, 10.0);

        let texture = engine
            .assets
            .load_texture(rl, &engine.thread, "pipe_n_ground.png")
            .expect("Pipe and ground png must be defined");

        let tubes = Tubes::new(texture.clone());
        let mut ground = Ground::new(texture, 60.0);
        ground.generate(engine);
        self.state = Some(State {
            player,
            tubes,
            ground,
        });
    }

    fn update(&mut self, engine: &Engine) {
        let state = self
            .state
            .as_mut()
            .expect("State must be loaded before update");

        if engine.rl.borrow().is_key_pressed(KeyboardKey::KEY_S) {
            state.tubes.spawn(engine, tube::Pos::Bottom, 0);
        }

        state.tubes.update(engine);
        state.player.update(engine);
        state.ground.update(engine);
    }

    fn draw(&mut self, _engine: &Engine, renderer: &mut RendererHandler) {
        const FZ: i32 = 10;
        renderer.draw(|d, r| {
            let state = self
                .state
                .as_ref()
                .expect("State must be loaded before draw");
            const TEXT: &str = "Press [Space] to jump";
            let x = r.width as i32 / 2 - d.measure_text(TEXT, FZ) / 2;
            let y = r.height as i32 / 2 - FZ / 2;
            d.draw_text(TEXT, x, y, FZ, Color::WHITE);
            state.tubes.draw(d);
            state.player.draw(d);
            state.ground.draw(d);
        });
    }
}
