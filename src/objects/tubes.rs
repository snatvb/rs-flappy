use crate::prelude::*;

use super::{ground, tube, Tube};

pub struct Tubes {
    pool: Vec<Tube>,
    pub active: Vec<Tube>,
    texture: Asset2D,
}

const LAYERS: u32 = ground::LAYERS as u32;
const TUBE_W: u32 = 32;
const TUBE_H: u32 = 48;

const TUBE_OFFSETS: &[f32] = &[8.0, 4.0, 0.0, -4.0, -8.0];

pub fn rand_tube_offset() -> f32 {
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
    #[inline(always)]
    pub fn spawn_double(&mut self, engine: &Engine, variant: u8, offset: f32) {
        self.spawn(engine, variant, tube::Pos::Top, offset);
        self.spawn(engine, variant, tube::Pos::Bottom, offset);
    }

    pub fn spawn(&mut self, engine: &Engine, variant: u8, pos: tube::Pos, offset: f32) {
        let (width, height) = engine.renderer.borrow().size();
        let y = match pos {
            tube::Pos::Bottom => (height - TUBE_H * LAYERS) as f32 + TUBE_H as f32 * 1.25 - offset,
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
                visited: false,

                pos,
            }
        });

        // tube.flip(match pos {
        //     tube::Pos::Bottom => sprite::Direction::Up,
        //     tube::Pos::Top => sprite::Direction::UpSide,
        // });
        tube.pos = pos;
        tube.set_position(x as f32, y);
        tube.visited = false;
        tube.set_variant(variant);

        self.active.push(tube);
        {
            let amount = self.active.len();
            log::debug!("Spawned tube, active: {amount}");
        }
    }

    pub fn update(&mut self) {
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
    }

    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        for tube in &self.active {
            tube.draw(d);
        }
    }
}
