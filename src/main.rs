extern crate sdl2; 

use std::io::Write;

// use std::thread::sleep;
use joeboid::boid::Boid;
use joeboid::BoidCanvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;

struct Wrapper(Canvas<Window>);

//NOTE: idea: have the title bar update with some info. use set_title() on the window
impl BoidCanvas for Wrapper {
    fn draw_triangle(&mut self, p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> Result<(), String> {
        self.0.draw_line(p1, p2)?;
        self.0.draw_line(p2, p3)?;
        self.0.draw_line(p3, p1)?;
        Ok(())
    }
}
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("boids", 1500, 800)
        .position_centered()
        .build()
        .unwrap();
 
    let  canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
    let mut canvas = Wrapper(canvas);
    canvas.0.set_draw_color(Color::RGB(0, 0, 0));
    canvas.0.clear();
    canvas.0.set_draw_color(Color::RGB(255, 255, 255));
    let _ = canvas.draw_triangle((4, 22), (66, 77), (99, 200));
    canvas.0.present();
    let bounds = canvas.0.output_size().unwrap().clone();
    let mut flock_master = Boid::new((bounds.0 as usize, bounds.1 as usize));
    flock_master.init_boidee_random(1000);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut flock_scare: Option<f32> = None;

    'running: loop {
        canvas.0.set_draw_color(Color::RGB(255, 255, 255));
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::Space){
            canvas.0.set_draw_color(Color::RGB(0, 0, 0));
            canvas.0.clear();
            canvas.0.set_draw_color(Color::RGB(255, 0, 0));
            flock_scare = match flock_scare{
                None => Some(-20.0),
                Some(v) => if v < -1.0{ Some(v + 1.0)}else{ Some(v) }
            } 
        }else{ // space is undepressed
            // if it's none, our job is already done
            if let Some(_) = flock_scare {
                flock_scare = None;
            }
            canvas.0.set_draw_color(Color::RGB(0, 0, 0));
            canvas.0.clear();
            canvas.0.set_draw_color(Color::RGB(255, 255, 255));
        }
        // canvas.0.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.0.clear();
        // canvas.0.set_draw_color(Color::RGB(255, 255, 255));
        flock_master.flock_scare(flock_scare);
        flock_master.step_draw(&mut canvas);
        // The rest of the game loop goes here...
        canvas.0.present();
    }
}
