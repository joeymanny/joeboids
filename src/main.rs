extern crate sdl2; 

use clap::Parser;
// use std::thread::sleep;
use joeboid::boid::Boid;
use joeboid::BoidCanvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::ops::{Deref, DerefMut};
#[derive(Parser)]
#[clap(author="Joseph Peterson", version, about="Joe's crummy boid project")]
struct Arguments{
    #[clap(short='n', long="num", default_value_t=200)]
    /// Number of bird-like-objects to conjure
    num: usize,
    #[clap(long="width", default_value_t=600)]
    /// Window width
    width: u32,
    #[clap(long="height", default_value_t=400)]
    /// Window width
    height: u32,

}
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
impl Deref for Wrapper{
    type Target = Canvas<Window>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Wrapper{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
pub fn main() {
    let config = Arguments::parse();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("boids", config.width, config.height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
 
    let  canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
    let mut canvas = Wrapper(canvas);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let _ = canvas.draw_triangle((4, 22), (66, 77), (99, 200));
    canvas.present();
    let bounds = canvas.output_size().unwrap().clone();
    let mut flock_master = Boid::new((bounds.0 as usize, bounds.1 as usize));
    flock_master.init_boidee_random(config.num);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut flock_scare: Option<f32> = None;

    'running: loop {
        // see if at any point they tried to leave the application
        // will be added to queue so it'll work even between checks
        for event in event_pump.poll_iter(){
            match event {
                // with (x) button
                Event::Quit {..} |
                // or esc key
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => ()
            }
        }
        // set draw color to white
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // if space is being pressed this frame, panic. won't be added to queue, 
        // only happens if it is being pressed RIGHT NOW. more responsvie
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::Space){
            // clear screen
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            // set draw color to red for panic mode
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            // update flock scare factor
            flock_scare = match flock_scare{
                // starts off at -20.0
                None => Some(-20.0),
                // decreases (increases) to -1.0
                Some(v) => if v < -1.0{ Some(v + 1.0)}else{ Some(v) }
            } 
        }else{ // space is undepressed
            // reset flock scare if it isn't already
            if let Some(_) = flock_scare {
                flock_scare = None;
            }
            // clear canvas
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            // set color to white for normal mode
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        }
        // tell the Boid what its new flock scare is
        flock_master.flock_scare(flock_scare);
        // step Boidees and draw to canvas
        flock_master.step_draw(&mut canvas);
        // render
        canvas.present();
    }
}
