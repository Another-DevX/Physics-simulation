use rand::Rng;
use sdl2::{
    event::Event,
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    Sdl,
};
use std::time::Instant;

// ======================
// Constantes equivalentes
// ======================
const SCREEN_WIDTH: i32 = 1080;
const SCREEN_HEIGHT: i32 = 720;

const GRAVITY: f32 = 980.0;
const ELASTICITY: f32 = 1.0;

const TRACE_LIMIT: usize = 20;

// ======================
// Structs
// ======================

#[derive(Clone)]
struct Trace {
    x: f32,
    y: f32,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: i32,
    traces: Vec<Trace>,
}

impl Particle {
    fn new(x: f32, y: f32, vx: f32, vy: f32, radius: i32) -> Self {
        let mut p = Particle {
            x,
            y,
            vx,
            vy,
            radius,
            traces: Vec::new(),
        };
        p.traces.push(Trace { x, y });
        p
    }

    fn set_position(&mut self, nx: f32, ny: f32) {
        self.x = nx;
        self.y = ny;
    }

    fn update(&mut self, dt: f32, enable_traces: bool, screen_w: i32, screen_h: i32) {
        // Gravedad
        self.vy += GRAVITY * dt;

        self.x += self.vx * dt;
        self.y += self.vy * dt;

        let r = self.radius as f32;
        if self.x - r < 0.0 {
            self.x = r;
            self.vx *= -ELASTICITY;
        } else if self.x + r > screen_w as f32 {
            self.x = (screen_w as f32) - r;
            self.vx *= -ELASTICITY;
        }

        if self.y - r < 0.0 {
            self.y = r;
            self.vy *= -ELASTICITY;
        } else if self.y + r > screen_h as f32 {
            self.y = (screen_h as f32) - r;
            self.vy *= -ELASTICITY;
        }

        if enable_traces {
            self.traces.push(Trace { x: self.x, y: self.y });
            if self.traces.len() > TRACE_LIMIT {
                self.traces.remove(0);
            }
        } else {
            self.traces.clear();
        }
    }

    fn collision(&mut self, vbx: f32, vby: f32, nx: f32, ny: f32, modn: f32) {
        let nhat_x = nx / modn;
        let nhat_y = ny / modn;

        // Vector tangente (perpendicular al normal)
        let that_x = -nhat_y;
        let that_y = nhat_x;

        let va_t = self.vx * that_x + self.vy * that_y;

        let vb_n = vbx * nhat_x + vby * nhat_y;

        let va_n_final = vb_n; 
        let va_t_final = va_t; 

        println!(
            "Before Collision: v1x = {}, v1y = {}, v2x = {}, v2y = {}",
            self.vx, self.vy, vbx, vby
        );

        self.vx = va_n_final * nhat_x + va_t_final * that_x;
        self.vy = va_n_final * nhat_y + va_t_final * that_y;

        println!(
            "After Collision: v1x = {}, v1y = {}, v2x = {}, v2y = {}",
            self.vx, self.vy, vbx, vby
        );
    }

    fn render<T: sdl2::render::RenderTarget>(&self, canvas: &mut sdl2::render::Canvas<T>, enable_traces: bool) {
        let x_i16 = self.x as i16;
        let y_i16 = self.y as i16;
        let r_i16 = self.radius as i16;

        let _ = canvas.filled_circle(x_i16, y_i16, r_i16, Color::RGBA(0, 255, 0, 255));

        if enable_traces {
            let size = self.traces.len();
            for (i, trace) in self.traces.iter().enumerate() {
                let scaling_factor = (i as f32 + 1.0) / size as f32;
                let scaled_radius = (self.radius as f32 * scaling_factor * 0.7) as i16;
                let tx = trace.x as i16;
                let ty = trace.y as i16;

                let _ = canvas.filled_circle(tx, ty, scaled_radius, Color::RGBA(0, 255, 0, 255));
            }
        }
    }
}

struct AppState {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    particles: Vec<Particle>,

    cell_size: i32,
    grid_cols: i32,
    grid_rows: i32,
    grid: Vec<Vec<Vec<usize>>>,

    simulation_speed: f32,
    quit: bool,
    paused: bool,
    enable_traces: bool,

    previous_instant: Instant,
    pause_instant: Instant,
}

// ======================
// Funciones de utilidad
// ======================

fn init_app() -> Result<AppState, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Crear ventana
    let window = video_subsystem
        .window("Particle interaction simulation (Rust/SDL2)",
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Crear canvas
    let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));

    let cell_size = 25;
    let grid_cols = (SCREEN_WIDTH / cell_size) + 1;
    let grid_rows = (SCREEN_HEIGHT / cell_size) + 1;

    // Grid 3D = [col][row] -> Vec<indices>
    let grid = vec![vec![Vec::<usize>::new(); grid_rows as usize]; grid_cols as usize];

    Ok(AppState {
        sdl_context,
        canvas,
        particles: Vec::new(),
        cell_size,
        grid_cols,
        grid_rows,
        grid,
        simulation_speed: 1.0,
        quit: false,
        paused: false,
        enable_traces: true,
        previous_instant: Instant::now(),
        pause_instant: Instant::now(),
    })
}

fn assign_particles_to_grid(state: &mut AppState) {
    for col in 0..state.grid_cols {
        for row in 0..state.grid_rows {
            state.grid[col as usize][row as usize].clear();
        }
    }

    for (i, p) in state.particles.iter().enumerate() {
        let cell_x = (p.x as i32) / state.cell_size;
        let cell_y = (p.y as i32) / state.cell_size;

        if cell_x >= 0 && cell_x < state.grid_cols && cell_y >= 0 && cell_y < state.grid_rows {
            state.grid[cell_x as usize][cell_y as usize].push(i);
        }
    }
}

fn check_collision_between(p1: &mut Particle, p2: &mut Particle) {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (r1, r2) = (p1.radius as f32, p2.radius as f32);

    let dx = x2 - x1;
    let dy = y2 - y1;
    let d_sq = dx * dx + dy * dy;
    let r_sum_sq = (r1 + r2) * (r1 + r2);

    if d_sq <= r_sum_sq {
        let dist = d_sq.sqrt();
        if dist < 1e-6 {
            return;
        }

        let nx = dx;
        let ny = dy;
        let overlap = (r1 + r2 - dist) / 2.0;

        let v1x = p1.vx;
        let v1y = p1.vy;
        let v2x = p2.vx;
        let v2y = p2.vy;

        p1.collision(v2x, v2y, nx, ny, dist);
        p2.collision(v1x, v1y, -nx, -ny, dist);

        let inv_dist = 1.0 / dist;
        let nx_norm = nx * inv_dist;
        let ny_norm = ny * inv_dist;

        let (p1x, p1y) = (x1 - nx_norm * overlap, y1 - ny_norm * overlap);
        let (p2x, p2y) = (x2 + nx_norm * overlap, y2 + ny_norm * overlap);

        p1.set_position(p1x, p1y);
        p2.set_position(p2x, p2y);
    }
}

fn check_collisions(state: &mut AppState) {
    let offsets = [[1, 0], [0, 1], [1, 1]];

    for col in 0..state.grid_cols {
        for row in 0..state.grid_rows {
            let cell_particles = &state.grid[col as usize][row as usize];
            for i in 0..cell_particles.len() {
                for j in (i + 1)..cell_particles.len() {
                    let idx1 = cell_particles[i];
                    let idx2 = cell_particles[j];
                    let (p1, p2) = {
                        let ptr1 = &mut state.particles[idx1] as *mut Particle;
                        let ptr2 = &mut state.particles[idx2] as *mut Particle;
                        unsafe { (&mut *ptr1, &mut *ptr2) }
                    };
                    check_collision_between(p1, p2);
                }
            }
            for off in &offsets {
                let nx = col + off[0];
                let ny = row + off[1];

                if nx < 0 || nx >= state.grid_cols || ny < 0 || ny >= state.grid_rows {
                    continue;
                }
                let neighbor_particles = &state.grid[nx as usize][ny as usize];

                for &idx1 in cell_particles {
                    for &idx2 in neighbor_particles {
                        let (p1, p2) = {
                            let ptr1 = &mut state.particles[idx1] as *mut Particle;
                            let ptr2 = &mut state.particles[idx2] as *mut Particle;
                            unsafe { (&mut *ptr1, &mut *ptr2) }
                        };
                        check_collision_between(p1, p2);
                    }
                }
            }
        }
    }
}

fn main_loop(state: &mut AppState) {
    let mut event_pump = match state.sdl_context.event_pump() {
        Ok(ep) => ep,
        Err(e) => {
            eprintln!("Error creando event pump: {}", e);
            return;
        }
    };

    'running: loop {
        // -------------------------
        // Proccess events
        // -------------------------
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    state.quit = true;
                }
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => {
                        state.quit = true;
                    }
                    Some(Keycode::Space) => {
                        if state.paused {
                            let now = Instant::now();
                            let paused_duration = now.duration_since(state.pause_instant);
                            state.previous_instant += paused_duration;
                        } else {
                            state.pause_instant = Instant::now();
                        }
                        state.paused = !state.paused;
                    }
                    Some(Keycode::Left) => {
                        state.simulation_speed -= 0.1;
                        if state.simulation_speed < -2.0 {
                            state.simulation_speed = -2.0;
                        }
                    }
                    Some(Keycode::Right) => {
                        state.simulation_speed += 0.1;
                        if state.simulation_speed > 2.0 {
                            state.simulation_speed = 2.0;
                        }
                    }
                    Some(Keycode::Down) => {
                        state.simulation_speed = -1.0;
                    }
                    Some(Keycode::Up) => {
                        state.simulation_speed = 1.0;
                    }
                    Some(Keycode::N) => {
                        let mut rng = rand::thread_rng();
                        for _ in 0..10 {
                            let px = rng.gen_range(0..SCREEN_WIDTH) as f32;
                            let py = rng.gen_range(0..SCREEN_HEIGHT) as f32;
                            let vx = (rng.gen_range(-200..200) as f32) / 1.5;
                            let vy = (rng.gen_range(-200..200) as f32) / 1.5;
                            let radius = 10;
                            state.particles.push(Particle::new(px, py, vx, vy, radius));
                        }
                    }
                    Some(Keycode::R) => {
                        state.particles.clear();
                    }
                    Some(Keycode::T) => {
                        state.enable_traces = !state.enable_traces;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if state.quit {
            break 'running;
        }

        // -------------------------
        // Lógica de simulación
        // -------------------------
        if !state.paused {
            let now = Instant::now();
            let dt = now.duration_since(state.previous_instant).as_secs_f32();
            state.previous_instant = now;

            let real_dt = dt * state.simulation_speed;
            for p in &mut state.particles {
                p.update(real_dt, state.enable_traces, SCREEN_WIDTH, SCREEN_HEIGHT);
            }

            assign_particles_to_grid(state);
            check_collisions(state);
        }

  
        state.canvas.set_draw_color(Color::RGB(0, 0, 0));
        state.canvas.clear();

        for p in &state.particles {
            p.render(&mut state.canvas, state.enable_traces);
        }

        state.canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn main() {
    let mut state = match init_app() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("No se pudo inicializar la app: {}", e);
            return;
        }
    };

    main_loop(&mut state);
}
