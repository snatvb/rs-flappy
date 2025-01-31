#![allow(dead_code)]
#![allow(unused_imports)]
use engine::Engine;
use raylib::prelude::*;
mod engine;

fn main() {
    let (rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    Engine::init(rl, thread);

    while !Engine::rl().window_should_close() {
        let mut rl = Engine::rl();
        let mut d = rl.begin_drawing(&Engine::rl_thread());

        d.clear_background(Color::WHITE);

        // drop(rl);

        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
