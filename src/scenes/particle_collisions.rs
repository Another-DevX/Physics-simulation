use crate::engine::{GlobalContext, Scene};
use crate::models::particle::Particle;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};

use rand::Rng;

pub struct ParticleCollisionScene {
    pub done: bool,
    pub particles: Vec<Particle>,
    pub enable_traces: bool,
    cell_size: u32,
    grid_cols: u32,
    grid_rows: u32,
    grid: Vec<Vec<Vec<usize>>>,
}

impl ParticleCollisionScene {
    pub fn new(ctx: &GlobalContext) -> Self {
        let cell_size = 25;
        let grid_cols = (ctx.screen_width / cell_size) + 1;
        let grid_rows = (ctx.screen_height / cell_size) + 1;

        // Grid 3D = [col][row] -> Vec<indices>
        let grid = vec![vec![Vec::<usize>::new(); grid_rows as usize]; grid_cols as usize];
        ParticleCollisionScene {
            done: false,
            particles: Vec::new(),
            enable_traces: true,
            cell_size,
            grid,
            grid_cols,
            grid_rows,
        }
    }
    fn assign_particles_to_grid(&mut self) {
        for col in 0..self.grid_cols {
            for row in 0..self.grid_rows {
                self.grid[col as usize][row as usize].clear();
            }
        }

        for (i, p) in self.particles.iter().enumerate() {
            let cell_x = (p.x as u32) / self.cell_size;
            let cell_y = (p.y as u32) / self.cell_size;

            if cell_x < self.grid_cols && cell_y < self.grid_rows {
                self.grid[cell_x as usize][cell_y as usize].push(i);
            }
        }
    }

    fn check_collisions(&mut self) {
        let offsets = [[1, 0], [0, 1], [1, 1]];
        let grid_cols = self.grid_cols.clone();
        let grid_rows = self.grid_rows.clone();
        let grid = self.grid.clone();

        for col in 0..grid_cols {
            for row in 0..grid_rows {
                let cell_particles = &grid[col as usize][row as usize];
                for i in 0..cell_particles.len() {
                    for j in (i + 1)..cell_particles.len() {
                        let idx1 = cell_particles[i];
                        let idx2 = cell_particles[j];
                        let (p1, p2) = {
                            let ptr1 = &mut self.particles[idx1] as *mut Particle;
                            let ptr2 = &mut self.particles[idx2] as *mut Particle;
                            unsafe { (&mut *ptr1, &mut *ptr2) }
                        };
                        self.check_collision_between(p1, p2);
                    }
                }
                for off in &offsets {
                    let nx = col + off[0];
                    let ny = row + off[1];

                    if nx >= grid_cols || ny >= grid_rows {
                        continue;
                    }
                    let neighbor_particles = &grid[nx as usize][ny as usize];

                    for &idx1 in cell_particles {
                        for &idx2 in neighbor_particles {
                            let (p1, p2) = {
                                let ptr1 = &mut self.particles[idx1] as *mut Particle;
                                let ptr2 = &mut self.particles[idx2] as *mut Particle;
                                unsafe { (&mut *ptr1, &mut *ptr2) }
                            };
                            self.check_collision_between(p1, p2);
                        }
                    }
                }
            }
        }
    }

    fn check_collision_between(&mut self, p1: &mut Particle, p2: &mut Particle) {
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

            self.collision(p1, p2, (v2x, v1x), (v2y, v1y), (nx, -nx), (ny, -ny), dist);

            let inv_dist = 1.0 / dist;
            let nx_norm = nx * inv_dist;
            let ny_norm = ny * inv_dist;

            let (p1x, p1y) = (x1 - nx_norm * overlap, y1 - ny_norm * overlap);
            let (p2x, p2y) = (x2 + nx_norm * overlap, y2 + ny_norm * overlap);

            p1.set_position(p1x, p1y);
            p2.set_position(p2x, p2y);
        }
    }

    fn collision(
        &mut self,
        p1: &mut Particle,
        p2: &mut Particle,
        vbx: (f32, f32),
        vby: (f32, f32),
        nx: (f32, f32),
        ny: (f32, f32),
        modn: f32,
    ) {
        let (vbx1, vbx2) = vbx;
        let (vby1, vby2) = vby;
        let (nx1, nx2) = nx;
        let (ny1, ny2) = ny;

        let nhat_x1 = nx1 / modn;
        let nhat_x2 = nx2 / modn;
        let nhat_y1 = ny1 / modn;
        let nhat_y2 = ny2 / modn;

        let that_x1 = -nhat_y1;
        let that_x2 = -nhat_y2;
        let that_y1 = nhat_x1;
        let that_y2 = nhat_x2;

        let va_t1 = p1.vx * that_x1 + p1.vy * that_y1;
        let va_t2 = p2.vx * that_x2 + p2.vy * that_y2;

        let vb_n1 = vbx1 * nhat_x1 + vby1 * nhat_y1;
        let vb_n2 = vbx2 * nhat_x2 + vby2 * nhat_y2;

        let va_n_final1 = vb_n1;
        let va_n_final2 = vb_n2;
        let va_t_final1 = va_t1;
        let va_t_final2 = va_t2;

        p1.vx = va_n_final1 * nhat_x1 + va_t_final1 * that_x1;
        p2.vx = va_n_final2 * nhat_x2 + va_t_final2 * that_x2;
        p1.vy = va_n_final1 * nhat_y1 + va_t_final1 * that_y1;
        p2.vy = va_n_final2 * nhat_y2 + va_t_final2 * that_y2;
    }
}

impl Scene for ParticleCollisionScene {
    fn update(&mut self, ctx: &mut GlobalContext, dt: f32) {
        if !ctx.paused {
            let real_dt = dt * ctx.simulation_speed;
            self.assign_particles_to_grid();
            self.check_collisions();
            for p in &mut self.particles {
                p.update(real_dt, ctx.screen_width, ctx.screen_height, self.enable_traces);
            }
        }
    }

    fn render(&mut self, _ctx: &GlobalContext, canvas: &mut Canvas<Window>) {
        for particle in &self.particles {
            particle.render(canvas, self.enable_traces);
        }

        let particle_count = self.particles.len();
        let text = format!("Total particles: {}", particle_count);
        let x: i16 = 10;
        let y: i16 = 10;

        let r = 255;
        let g = 255;
        let b = 255;
        let a = 255;

        let _ = canvas.string(x, y, &text, (r, g, b, a));
    }

    fn handle_event(&mut self, ctx: &mut GlobalContext, event: &Event) {
        if let Event::KeyDown {
            keycode: Some(k), ..
        } = event
        {
            match k {
                Keycode::Escape => {
                    self.done = true;
                }
                Keycode::Space => {
                    ctx.paused = !ctx.paused;
                }
                Keycode::Left => {
                    ctx.simulation_speed -= 0.1;
                }
                Keycode::Right => {
                    ctx.simulation_speed += 0.1;
                }
                Keycode::Down => {
                    ctx.simulation_speed = -1.0;
                }
                Keycode::Up => {
                    ctx.simulation_speed = 1.0;
                }
                Keycode::T => self.enable_traces = !self.enable_traces,
                Keycode::N => {
                    let mut rng = rand::thread_rng();
                    let px = rng.gen_range(0..ctx.screen_width) as f32;
                    let py = rng.gen_range(0..ctx.screen_height) as f32;
                    let vx = (rng.gen_range(-200..200) as f32) / 1.5;
                    let vy = (rng.gen_range(-200..200) as f32) / 1.5;
                    let radius = 10;
                    self.particles.push(Particle::new(px, py, vx, vy, radius))
                }
                _ => {}
            }
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }
}
