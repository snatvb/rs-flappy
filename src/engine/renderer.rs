use std::cell::RefCell;

use derive_more::{Deref, DerefMut};
use raylib::prelude::*;

use super::{scene::Scene, Engine};

#[derive(Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub source: Rectangle,
    pub dest: Rectangle,
    pub origin: Vector2,
    texture: RefCell<RenderTexture2D>,
}

impl Renderer {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        let w = width as f32;
        let h = height as f32;
        let screen_w = rl.get_screen_width() as f32;
        let screen_h = rl.get_screen_height() as f32;
        let scale = (screen_w / w).min(screen_h / h);
        let margin_x = (screen_w - (w * scale)) / 2f32;
        let margin_y = (screen_h - (h * scale)) / 2f32;
        let source = Rectangle::new(0f32, 0f32, w, -h);
        let dest = Rectangle::new(margin_x, margin_y, w * scale, h * scale);

        Ok(Self {
            source,
            dest,
            width,
            height,
            origin: Vector2::default(),
            texture: RefCell::new(rl.load_render_texture(thread, width, height)?),
        })
    }

    pub fn clean(&self, thread: &RaylibThread, d: &mut RaylibDrawHandle) {
        let texture = &mut *self.texture.borrow_mut();
        let mut d = d.begin_texture_mode(thread, texture);
        d.clear_background(Color::BLANK);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_pro(
            &*self.texture.borrow_mut(),
            self.source,
            self.dest,
            Vector2::default(),
            0f32,
            Color::WHITE,
        )
    }
}

pub struct RendererHandler<'a, 'b, 'd> {
    renderer: RefCell<&'a Renderer>,
    thread: RaylibThread,
    d: RefCell<&'b mut RaylibDrawHandle<'d>>,
}

impl<'a, 'b, 'd> RendererHandler<'a, 'b, 'd> {
    pub fn new(
        renderer: &'a mut Renderer,
        thread: RaylibThread,
        d: &'b mut RaylibDrawHandle<'d>,
    ) -> Self {
        Self {
            renderer: RefCell::new(renderer),
            d: RefCell::new(d),
            thread,
        }
    }

    pub fn width(&self) -> u32 {
        return self.renderer.borrow().width;
    }

    pub fn height(&self) -> u32 {
        return self.renderer.borrow().height;
    }

    pub fn draw<F>(&self, f: F)
    where
        F: FnOnce(&mut RaylibTextureMode<RaylibDrawHandle<'d>>, &Renderer),
    {
        let mut d = self.d.borrow_mut();
        let texture = &mut self.renderer.borrow_mut().texture.borrow_mut();
        let renderer = self.renderer.borrow();
        let mut d = d.begin_texture_mode(&self.thread, texture);
        f(&mut d, &renderer);
    }
}
