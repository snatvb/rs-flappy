use std::borrow::BorrowMut;

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
const TUBE_W: u32 = 32;
const TUBE_H: u32 = 48;

const TUBE_OFFSETS: &[f32] = &[8.0, 4.0, 0.0, -4.0, -8.0];
fn rand_tube_offset() -> f32 {
    let mut rng = rand::rng();
    TUBE_OFFSETS.choose(&mut rng).copied().unwrap_or(0.0)
}

impl Tubes {
    pub fn new(texture: Asset2D) -> Self {
        Self {
            texture,
            pool: Default::default(),
            active: Default::default(),
        }
    }

    // TODO: Change variant
    pub fn spawn(&mut self, enigne: &Engine, pos: tube::Pos, offset: f32) {
        let variant = 0;

        let (width, height) = enigne.renderer.borrow().size();
        let y = match pos {
            tube::Pos::Bottom => (height - TUBE_W * (LAYERS - 1) + 24) as f32 - offset,
            tube::Pos::Top => {
                TUBE_OFFSETS
                    .iter()
                    .copied()
                    .fold(f32::NEG_INFINITY, |a, b| a.max(b))
                    * -1.0
                    - 8.0 // opened part of the tube
                    - offset
            }
        };

        let x = width + TUBE_W;

        let mut tube = self.pool.pop().unwrap_or_else(|| {
            log::debug!("Create new tube {pos} {variant}");

            Tube {
                sprite: Sprite::new(self.texture.clone(), TUBE_W as f32, TUBE_H as f32),
                variant,
                pos,
            }
        });

        // tube.flip(match pos {
        //     tube::Pos::Bottom => sprite::Direction::Up,
        //     tube::Pos::Top => sprite::Direction::UpSide,
        // });
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
    tube_spawn_timer: Timer,
}

pub struct Game {
    state: Option<State>,
    rng: rand::rngs::ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: None,
            rng: rand::rng(),
        }
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
            tube_spawn_timer: Timer {
                current: 0.8,
                max: 1.0,
            },
        });
    }

    fn update(&mut self, engine: &Engine) {
        let state = self
            .state
            .as_mut()
            .expect("State must be loaded before update");

        if engine.rl.borrow().is_key_pressed(KeyboardKey::KEY_S) {
            state.tubes.spawn(engine, tube::Pos::Bottom, 0.0);
        }

        if state.tube_spawn_timer.tick(engine.delta.get()) {
            let delay = self.rng.random_range(1.2..1.7);
            state.tube_spawn_timer.max = delay;
            let offset = rand_tube_offset();
            state.tubes.spawn(engine, tube::Pos::Top, offset);
            state.tubes.spawn(engine, tube::Pos::Bottom, offset);
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
