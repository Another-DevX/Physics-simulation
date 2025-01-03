mod engine;
mod models;
mod scenes;
mod utils;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use engine::Engine;
use engine::Scene;
use scenes::lorenz_attractor::LorenzAttractor;
use scenes::pendulum::Pendulum;
use scenes::particle_collisions::ParticleCollisionScene;

fn scene_loader() -> usize {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a simulation:")
        .items(&["Particle Collisions", "Lorenz Attractor", "Pendulum"])
        .default(0)
        .interact()
        .unwrap();
    match selection {
        0 => println!("Loading Particle Collision Scene..."),
        1 => println!("Loading Lorenz Attractor..."),
        2 => println!("Loading Pendulum Scene..."),
        _ => println!("Invalid selection."),
    }
    selection
}

fn main() -> Result<(), String> {
    const WINDOW_WIDTH: u32 = 2048;
    const WINDOW_HEIGHT: u32 = 1280;
    let window_title = "Particle Simulation in Rust";
    let selection = scene_loader();
    let mut engine = Engine::new(window_title, WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let mut options: Vec<Box<dyn Scene>> = vec![
        Box::new(ParticleCollisionScene::new(&engine.global_context)),
        Box::new(LorenzAttractor::new()),
        Box::new(Pendulum::new()),
    ];
    let mut selected_scene = options.remove(selection);

    engine.run(&mut *selected_scene);

    Ok(())
}
