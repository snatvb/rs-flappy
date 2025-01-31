#![allow(dead_code)]
#![allow(unused_imports)]
use engine::Engine;
use raylib::prelude::*;
mod engine;

fn main() {
    let (rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    let mut engine = Engine::new(rl, thread);
    engine.lock_fps(60);

    while !engine.should_close() {
        engine.tick()
    }
}
