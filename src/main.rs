mod engine;
mod models;
mod scenes;
mod utils;

use engine::Engine;
use scenes::particle_collisions::ParticleCollisionScene;
use scenes::lorenz_attractor::LorenzAttractor;

fn main() -> Result<(), String> {
    const WINDOW_WIDTH: u32 = 1080;
    const WINDOW_HEIGHT: u32 = 720;
    let window_title = "Particle Simulation in Rust";

    let mut engine = Engine::new(window_title, WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let mut particle_collision_scene = ParticleCollisionScene::new(&engine.global_context);
    let mut lorenz_attractor = LorenzAttractor::new();

    engine.run(&mut lorenz_attractor);

    Ok(())
}
