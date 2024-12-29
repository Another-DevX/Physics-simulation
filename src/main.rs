mod engine;
mod models;
mod scenes;
mod utils;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use engine::Engine;
use engine::Scene;
use scenes::lorenz_attractor::LorenzAttractor;
use scenes::particle_collisions::ParticleCollisionScene;

fn scene_loader() -> usize {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a simulation:")
        .items(&["Particle Collisions", "Lorenz Attractor"])
        .default(0)
        .interact()
        .unwrap();
    match selection {
        0 => println!("Cargando Particle Collision Scene..."),
        1 => println!("Cargando Lorenz Attractor..."),
        _ => println!("Selección inválida."),
    }
    selection
}

fn main() -> Result<(), String> {
    const WINDOW_WIDTH: u32 = 1080;
    const WINDOW_HEIGHT: u32 = 720;
    let window_title = "Particle Simulation in Rust";
    let selection = scene_loader();
    let mut engine = Engine::new(window_title, WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let mut options: Vec<Box<dyn Scene>> = vec![
        Box::new(ParticleCollisionScene::new(&engine.global_context)),
        Box::new(LorenzAttractor::new()),
    ];
    let mut selected_scene = options.remove(selection);

    engine.run(&mut *selected_scene);

    Ok(())
}
