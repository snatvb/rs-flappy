#![allow(dead_code)]
#![allow(unused_imports)]
extern crate raylib;
use std::env;

use engine::Engine;
use raylib::prelude::*;
mod engine;
mod objects;
mod prelude;
mod scenes;

fn main() {
    unsafe {
        env::set_var("LOG", env::var("LOG").unwrap_or("info".to_owned()));
    }
    env_logger::init();
    log::info!("Getting start");
    let (rl, thread) = raylib::init().size(800, 600).title("Hello, World").build();
    let mut engine = Engine::new(rl, thread).unwrap();
    engine
        .lock_fps(120)
        .register_scene(Box::new(scenes::Welcome::new()))
        .register_scene(Box::new(scenes::Game::new()))
        .switch_scene("welcome");

    while !engine.should_close() {
        engine.tick()
    }
}
