#![allow(dead_code)]
#![allow(unused_imports)]
extern crate raylib;
use engine::Engine;
use raylib::prelude::*;
mod engine;
mod scenes;

fn main() {
    let (rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    let mut engine = Engine::new(rl, thread).unwrap();
    engine
        .lock_fps(30)
        .register_scene(Box::new(scenes::Welcome::new()))
        .switch_scene("welcome");

    while !engine.should_close() {
        engine.tick()
    }
}
