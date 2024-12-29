use crate::engine::{GlobalContext, Scene};
use crate::utils::RK4::rk4;
use sdl2::sys::KeyCode;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::{render::Canvas, video::Window};

pub struct LorenzAttractor {
    sigma: f32,
    beta: f32,
    rho: f32,
    solutions: Option<(Vec<f32>, Vec<Vec<f32>>)>,
    current_index: usize,
    done: bool,
    camera_rotation: (f32, f32),
    is_mouse_down: bool,
    zoom: f32,
}

impl LorenzAttractor {
    pub fn new() -> Self {
        let mut lorenz_attractor = LorenzAttractor {
            sigma: 10.0,
            beta: 2.667,
            rho: 28.0,
            solutions: None,
            current_index: 0,
            done: false,
            camera_rotation: (0.0, 0.0),
            is_mouse_down: false,
            zoom: 1.0,
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

    fn rotate_3d(&self, point: (f32, f32, f32)) -> (f32, f32, f32) {
        let (x, y, z) = point;
        let (theta_x, theta_y) = self.camera_rotation;

        let (x1, y1, z1) = (
            x,
            y * theta_x.cos() - z * theta_x.sin(),
            y * theta_x.sin() + z * theta_x.cos(),
        );

        let (x2, y2, z2) = (
            x1 * theta_y.cos() + z1 * theta_y.sin(),
            y1,
            -x1 * theta_y.sin() + z1 * theta_y.cos(),
        );

        (x2, y2, z2)
    }

    fn project(&self, point: (f32, f32, f32), width: u32, height: u32) -> (i32, i32) {
        let (x, y, z) = point;

        let d = 50.0;
        let x_screen = x / (z + d) * 200.0 * self.zoom + width as f32 / 2.0;
        let y_screen = y / (z + d) * 200.0 * self.zoom + height as f32 / 2.0;

        (x_screen as i32, y_screen as i32)
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
            let mut last_projected = None;

            let total_points = solutions.len();

            for i in 1..self.current_index {
                let point = (solutions[i][0], solutions[i][1], solutions[i][2]);

                let rotated = self.rotate_3d(point);
                let projected = self.project(rotated, width, height);
                let t = i as f32 / total_points as f32;
                let r = (255.0 * (1.0 - t)) as u8;
                let g = (255.0 * t) as u8;

                if let Some(last) = last_projected {
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, 0));
                    let _ = canvas.draw_line(last, projected);
                }
                last_projected = Some(projected);
            }
        }
    }

    fn handle_event(&mut self, ctx: &mut GlobalContext, event: &Event) {
        match event {
            Event::MouseButtonDown { mouse_btn, .. } => {
                if *mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.is_mouse_down = true;
                }
            }
            Event::MouseButtonUp { mouse_btn, .. } => {
                if *mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.is_mouse_down = false;
                }
            }
            Event::MouseMotion { xrel, yrel, .. } => {
                if self.is_mouse_down {
                    self.camera_rotation.0 += *yrel as f32 * 0.01;
                    self.camera_rotation.1 += *xrel as f32 * 0.01;
                }
            }
            Event::KeyDown {
                keycode: Some(k),
                ..
            } => {
                match k {
                    Keycode::Escape => {
                        self.done = true;
                    },
                    Keycode::R => {
                        self.current_index = 0;
                    }
                    Keycode::Left => {
                        ctx.simulation_speed -= 0.1;
                    }
                    Keycode::Right => {
                        ctx.simulation_speed += 0.1;
                    }
                    _ => {}
                }
            }
            Event::MouseWheel { y, .. } => {
                if *y > 0 {
                    self.zoom *= 1.1; 
                } else if *y < 0 {
                    self.zoom *= 0.9; 
                }
            }
            _ => {}
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }
}
