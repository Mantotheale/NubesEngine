use crate::engine::Engine;
use crate::engine::game::Game;

mod engine;

fn main() {
    let engine: Engine<Game> = Engine::new();
    engine.run();
}
