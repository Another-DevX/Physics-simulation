use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

const GRAVITY: f32 = 980.0;
const ELASTICITY: f32 = 1.0;
const TRACE_LIMIT: usize = 20;

#[derive(Clone)]
pub struct Trace {
    pub x: f32,
    pub y: f32,
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: i32,
    pub traces: Vec<Trace>,
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, radius: i32) -> Self {
        let particle = Particle {
            x,
            y,
            vx,
            vy,
            radius,
            traces: vec![Trace { x, y }],
        };
        particle
    }

    pub fn update(&mut self, dt: f32, screen_w: u32, screen_h: u32, enable_traces: bool) {
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
            self.traces.push(Trace {
                x: self.x,
                y: self.y,
            });
            if self.traces.len() > TRACE_LIMIT {
                self.traces.remove(0);
            }
        } else {
            self.traces.clear();
        }
    }

    pub fn render<T: sdl2::render::RenderTarget>(
        &self,
        canvas: &mut sdl2::render::Canvas<T>,
        enable_traces: bool,
    ) {
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

    pub fn set_position(&mut self, nx: f32, ny: f32) {
        self.x = nx;
        self.y = ny;
    }
}
