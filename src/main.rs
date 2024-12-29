mod engine;
mod models;
mod scenes;

use engine::Engine;
use scenes::particle_collisions::ParticleCollisionScene;

fn main() -> Result<(), String> {
    const WINDOW_WIDTH: u32 = 1080;
    const WINDOW_HEIGHT: u32 = 720;
    let window_title = "Particle Simulation in Rust";

    let mut engine = Engine::new(window_title, WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let mut particle_collision_scene = ParticleCollisionScene::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    engine.run(&mut particle_collision_scene);

    Ok(())
}
