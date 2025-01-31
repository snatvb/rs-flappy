use std::collections::HashMap;

use raylib::prelude::*;

use self::scene::Scene;

mod renderer;
mod scene;

pub struct Engine {
    rl: RaylibHandle,
    thread: RaylibThread,
    scenes: HashMap<String, Box<dyn Scene>>,
}

impl Engine {
    pub fn new(rl: RaylibHandle, thread: RaylibThread) -> Engine {
        Self {
            rl,
            thread,
            scenes: HashMap::new(),
        }
    }

    pub fn lock_fps(&mut self, fps: u32) -> &mut Self {
        self.rl.set_target_fps(fps);
        self
    }

    pub fn should_close(&self) -> bool {
        self.rl.window_should_close()
    }

    pub fn register_scene(&mut self, scene: Box<dyn Scene>) -> &mut Self {
        self.scenes.insert(scene.name().to_owned(), scene);
        self
    }

    pub fn tick(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);

        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
