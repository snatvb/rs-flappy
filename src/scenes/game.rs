use std::borrow::BorrowMut;

use num::Integer;
use rand::prelude::IndexedRandom;

use crate::engine::physics::hit_test_rects;
use crate::engine::Engine;
use crate::objects::{ground, tube, tubes, Ground, Player, Tubes};
use crate::prelude::*;

#[derive(Debug)]
struct Score {
    amount: u32,
    scale: f32,
    color: Color,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            scale: 1.0,
            amount: 0,
            color: Color::WHITE,
        }
    }
}

impl Score {
    #[inline]
    pub fn update(&mut self) {
        self.scale = lerp(self.scale, 1.0, 0.3);
        self.color = color_lerp(self.color, Color::WHITE, 0.05);
    }

    #[inline]
    pub fn increment(&mut self) {
        self.amount += 1;
        self.scale = 2.0;
        if self.amount.mod_floor(&5_u32) == 0 {
            self.color = Color {
                r: 200,
                g: 10,
                b: 10,
                a: 255,
            };
            self.scale = 3.5;
        }
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>, r: &Renderer) {
        let fz = (10.0 * self.scale) as i32;
        let text = format!("{}", self.amount);
        let x = r.width as i32 / 2 - d.measure_text(&text, fz) / 2;
        let y = 24 - fz / 2;
        d.draw_text(&text, x, y, fz, self.color);
    }
}

struct State {
    player: Player,
    tubes: Tubes,
    ground: Ground,
    tube_spawn_timer: Timer,
    background: Sprite,
    world_bounds: Rectangle,
    score: Score,
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

    fn player_hits(state: &mut State) -> bool {
        if !hit_test_rects(&state.player.collider, &state.world_bounds) {
            return true;
        }
        for tube in &state.tubes.active {
            if hit_test_rects(&state.player.collider, tube.sprite.display_rect()) {
                return true;
            }
        }

        false
    }
}

impl Scene for Game {
    fn name(&self) -> &str {
        "game"
    }

    fn load(&mut self, engine: &Engine) {
        let rl = &mut engine.rl.borrow_mut();
        let renderer = engine.renderer.borrow();
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

        let texture = engine
            .assets
            .load_texture(rl, &engine.thread, "background/background1.png")
            .expect("Pipe and ground png must be defined");

        let (w, h) = (texture.width() as f32, texture.height() as f32);
        let background = Sprite::new(texture, w, h);

        self.state = Some(State {
            player,
            tubes,
            ground,
            score: Score::default(),
            world_bounds: Rectangle {
                x: 0.0,
                y: 0.0,
                width: renderer.width as f32,
                height: renderer.height as f32 - ground::LAYERS as f32 * ground::TILE_H,
            },
            background,
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

        if engine.rl.borrow().is_key_pressed(KeyboardKey::KEY_SPACE) {
            state.player.jump();
        }

        if state.tube_spawn_timer.tick(engine.delta.get()) {
            let delay = self.rng.random_range(1.2..1.7);
            state.tube_spawn_timer.max = delay;
            let offset = tubes::rand_tube_offset();
            let mut rng = rand::rng();
            let variant = rng.random_range(0..3);
            state.tubes.spawn_double(engine, variant, offset);
        }

        state.tubes.update();
        state.player.update(engine);
        state.ground.update(engine);
        state.player.sync_collider();

        if Game::player_hits(state) {
            engine.switch_scene("welcome");
        }

        state
            .tubes
            .active
            .iter_mut()
            .filter(|t| !t.visited && t.pos == tube::Pos::Bottom)
            .for_each(|tube| {
                if tube.x() < state.player.x() {
                    tube.visited = true;
                    state.score.increment();
                }
            });

        state.score.update();
    }

    fn draw(&mut self, _engine: &Engine, renderer: &mut RendererHandler) {
        const FZ: i32 = 10;
        renderer.draw(|d, r| {
            let state = self
                .state
                .as_ref()
                .expect("State must be loaded before draw");
            state.background.draw(d);

            const TEXT: &str = "Press [Space] to jump";
            let x = r.width as i32 / 2 - d.measure_text(TEXT, FZ) / 2;
            let y = r.height as i32 / 2 - FZ / 2;
            d.draw_text(TEXT, x, y, FZ, Color::WHITE);
            state.tubes.draw(d);
            state.player.draw(d);
            state.ground.draw(d);
            state.score.draw(d, r);

            state.player.draw_gizmoz(d);
            for tube in &state.tubes.active {
                tube.draw_gizmoz(d);
            }
        });
    }
}
