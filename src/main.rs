extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

struct Wrapper(Canvas<Window>)

//TODO
impl BoidCanvas for Wrapper {
    fn draw_triangle(&self, p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> Result<(), String> {
        self.0.draw_line(p1, p2)?;
        self.0.draw_line(p2, p3)?;
        self.0.draw_line(p3, p1)?;
    }
}
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut canvas = Wrapper(canvas);
    canvas.0.set_draw_color(Color::RGB(0, 0, 0));
    canvas.0.clear();
    canvas.0.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.init_boid(30);
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.boid_step();
        canvas.0.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
