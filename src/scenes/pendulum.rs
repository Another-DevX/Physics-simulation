use crate::engine::{GlobalContext, Scene};
use crate::utils::RK4::rk42nd_order;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::{render::Canvas, video::Window};
use std::f64::consts::PI;

pub struct Pendulum {
    solutions: Option<(Vec<f32>, Vec<f32>, Vec<f32>)>,
    current_index: usize,
    length: f32,
    gravity: f32,
}

impl Pendulum {
    pub fn new() -> Self {
        let mut pendulum = Pendulum {
            solutions: None,
            current_index: 0,
            gravity: 9.8,
            length: 2.0,
        };
        pendulum.solve();
        pendulum
    }

    fn domega(&self, _t: f32, thetha: f32, _omega: f32) -> f32 {
        (self.gravity / self.length) * f32::sin(thetha)
    }

    pub fn solve(&mut self) {
        let thetha0 = PI as f32 / 2.0;
        let omega0 = 0.0;

        let (a, b) = (0.0, 50.0);
        let n: u32 = 10000;

        fn dtheta(_t: f32, _thetha: f32, omega: f32) -> f32 {
            omega
        }

        let solutions = rk42nd_order(
            a,
            b,
            thetha0,
            omega0,
            dtheta as fn(f32, f32, f32) -> f32,
            |t, gamma, omega| self.domega(t, gamma, omega),
            n,
        );
        for value in &solutions.1 {
            print!("Solution theta:{} \n", value);
        }
        self.solutions = Some(solutions)
    }
}

impl Scene for Pendulum {
    fn handle_event(&mut self, _ctx: &mut GlobalContext, _event: &Event) {
        // Handle events here
    }

    fn update(&mut self, ctx: &mut GlobalContext, _dt: f32) {
        if self.solutions.is_none() {
            self.solve();
        }
        if let Some(sol) = &self.solutions {
            if (self.current_index + 1 + (ctx.simulation_speed * 10.) as usize) < sol.1.len() {
                self.current_index += 1 + (ctx.simulation_speed * 10.) as usize;
            } else {
                self.current_index = sol.1.len() - 1
            }
        }
    }

    fn render(&mut self, _ctx: &GlobalContext, canvas: &mut Canvas<Window>) {
        if let Some(sol) = &self.solutions {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            let (width, height) = canvas.output_size().unwrap_or((800, 600));
            let window_center_x = (width / 2) as i32;
            let window_center_y = (height / 2) as i32;
            let scale = 200.0;
            let solutions = &sol.1;
            let theta = solutions[self.current_index];

            let x = scale * self.length * theta.sin();
            let y = -scale * self.length * theta.cos();

            let point = (window_center_x + x as i32, window_center_y + y as i32);
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            canvas.draw_line((window_center_x, window_center_y), (point.0, point.1));
            canvas.filled_circle(point.0 as i16, point.1 as i16, 30, Color::RGB(0, 255, 0));
            // }
        }
    }

    fn is_done(&self) -> bool {
        // Determine if the scene is done
        false
    }
}
