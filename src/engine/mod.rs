use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell, RefMut},
    collections::HashMap,
    os::unix::ffi,
    rc::Rc,
};

use derive_more::{Deref, DerefMut};
use raylib::prelude::*;

pub use self::renderer::{Renderer, RendererHandler};
pub mod assets;
pub mod core;
pub mod renderer;
pub mod scene;

pub use assets::*;
pub use core::*;
pub use scene::Scene;

pub struct Engine {
    pub rl: RefCell<RaylibHandle>,
    pub thread: RaylibThread,
    pub renderer: RefCell<Renderer>,
    pub delta: Cell<f32>,
    pub assets: assets::Assets,
    scenes: RefCell<HashMap<String, Box<dyn Scene>>>,
    current_scene: RefCell<Option<String>>,
}

impl Engine {
    pub fn new(mut rl: RaylibHandle, thread: RaylibThread) -> Result<Engine, String> {
        let width = (rl.get_screen_width() / 4) as u32;
        let height = (rl.get_screen_height() / 4) as u32;
        let renderer = Renderer::new(&mut rl, &thread, width, height)?;
        let rl = RefCell::new(rl);

        Ok(Self {
            delta: Default::default(),
            rl,
            thread,
            assets: assets::Assets::new("assets"),
            renderer: RefCell::new(renderer),
            current_scene: RefCell::new(None),
            scenes: RefCell::new(HashMap::new()),
        })
    }

    pub fn lock_fps(&mut self, fps: u32) -> &mut Self {
        self.rl.borrow_mut().set_target_fps(fps);
        self
    }

    pub fn should_close(&self) -> bool {
        self.rl.borrow_mut().window_should_close()
    }

    pub fn register_scene(&mut self, scene: Box<dyn Scene>) -> &mut Self {
        self.scenes
            .borrow_mut()
            .insert(scene.name().to_owned(), scene);
        self
    }

    #[inline(always)]
    pub fn switch_scene(&self, name: &str) -> bool {
        self.send_switch_scene(name, None)
    }

    pub fn send_switch_scene(&self, name: &str, msg: Option<Box<dyn scene::SceneMessage>>) -> bool {
        if let Some(mut scene) = self.current_scene() {
            log::info!("Unlaading scene {name}...");
            scene.unload(self);
        }

        if let Some(mut scene) = SceneGuard::new(self, name) {
            log::info!("Loading scene {name}...");
            if let Some(msg) = msg {
                scene.preload_message(self, msg);
            }
            scene.load(self);
            self.current_scene.replace_with(|_| Some(name.to_owned()));
            return true;
        }
        log::warn!("Failed to load scene {name}");

        false
    }

    fn current_scene(&self) -> Option<SceneGuard> {
        self.current_scene
            .borrow()
            .as_ref()
            .and_then(|s| SceneGuard::new(self, s))
    }

    pub fn tick(&self) {
        self.delta.set(self.rl.borrow().get_frame_time());

        if let Some(mut scene) = self.current_scene() {
            scene.update(self);
        }

        let mut rl = self.rl.borrow_mut();
        let mut d = rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        let mut renderer = self.renderer.borrow_mut();
        renderer.clean(&self.thread, &mut d);
        let mut handler = RendererHandler::new(&mut renderer, self.thread.clone(), &mut d);
        if let Some(mut scene) = self.current_scene() {
            scene.draw(self, &mut handler);
        }
        renderer.draw(&mut d);
    }
}

struct SceneGuard<'a> {
    engine: &'a Engine,
    scene: Option<Box<dyn Scene>>,
}

impl<'a> SceneGuard<'a> {
    fn new(engine: &'a Engine, name: &str) -> Option<Self> {
        let mut scenes = engine.scenes.borrow_mut();
        scenes.remove(name).map(|scene| SceneGuard {
            engine,
            scene: Some(scene),
        })
    }
}

impl std::ops::Deref for SceneGuard<'_> {
    type Target = Box<dyn Scene>;

    fn deref(&self) -> &Self::Target {
        self.scene.as_ref().expect("Scene in guard must be defined")
    }
}

impl std::ops::DerefMut for SceneGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.scene.as_mut().expect("Scene in guard must be defined")
    }
}

impl Drop for SceneGuard<'_> {
    fn drop(&mut self) {
        self.engine.scenes.borrow_mut().insert(
            self.scene.as_ref().unwrap().name().to_owned(),
            self.scene.take().unwrap(),
        );
    }
}
