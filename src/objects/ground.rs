use crate::prelude::*;

pub struct Ground {
    tiles: Vec<Sprite>,
    texture: Asset2D,
}

const TILE_W: f32 = 32 as f32;
const TILE_H: f32 = 16 as f32;
const TEXTURE_Y_G: f32 = 64.0;
const TEXTURE_Y: f32 = TEXTURE_Y_G - TILE_H;
const TEXTURE_X: [f32; 2] = [0.0, TILE_W];
const LAYERS: i32 = 2;

// TODO: Add more layers
impl Ground {
    pub fn new(texture: Asset2D) -> Self {
        Self {
            tiles: Default::default(),
            texture,
        }
    }

    pub fn generate(&mut self, engine: &Engine) {
        self.tiles.clear();

        let mut rng = rand::rng();
        let renderer = engine.renderer.borrow();
        let amount = renderer.width / TILE_W as u32 + TILE_W as u32;

        for i in 0..amount {
            for l in 0..LAYERS {
                let x = i as f32 * TILE_W - TILE_W;
                let y = renderer.height as f32 - TILE_H * (l + 1) as f32;
                let mut sprite = Sprite::new(self.texture.clone(), TILE_W, TILE_H);
                sprite.set_position(x, y);
                let texture_y = if l == LAYERS - 1 {
                    TEXTURE_Y
                } else {
                    TEXTURE_Y_G
                };
                sprite.set_offset(*TEXTURE_X.choose(&mut rng).unwrap(), texture_y);
                self.tiles.push(sprite);
            }
        }
    }

    #[inline]
    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
        for tile in self.tiles.iter() {
            tile.draw(d);
        }
    }
}
