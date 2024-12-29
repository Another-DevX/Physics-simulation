use sdl2::{event::Event, pixels::Color, render::Canvas, video::Window, Sdl};

use std::time::Instant;

pub trait Scene {
    fn handle_event(&mut self, ctx: &mut GlobalContext, event: &Event);
    fn update(&mut self, ctx: &mut GlobalContext, dt: f32);
    fn render(&mut self, ctx: &GlobalContext, canvas: &mut Canvas<Window>);
    fn is_done(&self) -> bool;
}

pub struct GlobalContext {
    pub simulation_speed: f32,
    pub paused: bool,
    pub screen_width: u32,
    pub screen_height: u32,
}

pub struct Engine {
    _sdl_context: Sdl,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    previous_instant: Instant,
    pub global_context: GlobalContext,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let _sdl_context = sdl2::init()?;
        let video_subsystem = _sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = _sdl_context.event_pump()?;

        let global_context = GlobalContext {
            paused: false,
            simulation_speed: 1.0,
            screen_height: height,
            screen_width: width
        };

        Ok(Engine {
            _sdl_context,
            canvas,
            event_pump,
            previous_instant: Instant::now(),
            global_context,
        })
    }

    pub fn run<S: Scene + ?Sized>(&mut self, scene: &mut S) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'running;
                }
                scene.handle_event(&mut self.global_context, &event);
            }

            if scene.is_done() {
                break 'running;
            }

            let now = Instant::now();
            let dt = now.duration_since(self.previous_instant).as_secs_f32();
            self.previous_instant = now;

            scene.update(&mut self.global_context, dt);

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            scene.render(&mut self.global_context, &mut self.canvas);
            self.canvas.present();

            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
