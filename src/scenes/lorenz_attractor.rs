use crate::engine::{GlobalContext, Scene};
use crate::utils::RK4::rk4;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::{render::Canvas, video::Window};

pub struct LorenzAttractor {
    sigma: f32,
    beta: f32,
    rho: f32,
    current_coordinates: (f32, f32, f32),
    solutions: Option<(Vec<f32>, Vec<Vec<f32>>)>,
    current_index: usize,
    done: bool,
}

impl LorenzAttractor {
    pub fn new() -> Self {
        let mut lorenz_attractor = LorenzAttractor {
            sigma: 10.0,
            beta: 2.667,
            rho: 28.0,
            current_coordinates: (0., 0., 0.),
            solutions: None,
            current_index: 0,
            done: false,
        };
        lorenz_attractor.solve();
        lorenz_attractor
    }

    fn lorenz(&self, _t: f32, state: &[f32; 3]) -> [f32; 3] {
        let (x, y, z) = (state[0], state[1], state[2]);
        let x_dot = self.sigma * (y - x);
        let y_dot = x * (self.rho - z) - y;
        let z_dot = x * y - self.beta * z;
        [x_dot, y_dot, z_dot]
    }

    pub fn solve(&mut self) {
        let r0 = [0.0, 1.0, 1.05];
        let (a, b) = (0.0, 50.0);
        let n: u32 = 10000;
        let solutions = rk4(a, b, r0, |t, state| self.lorenz(t, state), n);
        for value in &solutions.1 {
            print!("Solution x:{}, y:{}, z:{} \n", value[0], value[1], value[2]);
        }
        self.solutions = Some(solutions);
    }
}

impl Scene for LorenzAttractor {
    fn update(&mut self, ctx: &mut GlobalContext, _dt: f32) {
        if self.solutions.is_none() {
            self.solve();
        }

        if let Some((_, ref sol)) = self.solutions {
            if (self.current_index + 1 + (ctx.simulation_speed * 10.) as usize) < sol.len() {
                self.current_index += 1 + (ctx.simulation_speed * 10.) as usize;
            } else {
                self.current_index = sol.len() - 1
            }
        }
    }

    fn render(&mut self, _ctx: &GlobalContext, canvas: &mut Canvas<Window>) {
        if let Some((_, ref solutions)) = self.solutions {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

            let (width, height) = canvas.output_size().unwrap_or((800, 600));
            let total_points = solutions.len();

            for i in 1..self.current_index {
                let prev = &solutions[i - 1];
                let curr = &solutions[i];

                let x1 = (prev[0] * 10.0 + width as f32 / 2.0) as i32;
                let y1 = (prev[1] * 10.0 + height as f32 / 2.0) as i32;

                let x2 = (curr[0] * 10.0 + width as f32 / 2.0) as i32;
                let y2 = (curr[1] * 10.0 + height as f32 / 2.0) as i32;

                let t = i as f32 / total_points as f32; 
                let r = (255.0 * (1.0 - t)) as u8; 
                let g = (255.0 * t) as u8;

                canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, 0));
                let _ = canvas.draw_line((x1, y1), (x2, y2));
            }
        }
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
                Keycode::Left => {
                    ctx.simulation_speed -= 0.1;
                }
                Keycode::Right => {
                    ctx.simulation_speed += 0.1;
                }
                Keycode::R => {
                    self.current_index = 0;
                }
                _ => {}
            }
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }
}
