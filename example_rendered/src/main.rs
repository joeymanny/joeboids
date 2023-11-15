// for file handling
use std::fs::File;
use std::io::BufWriter;

// for getting command line arguments
use clap::Parser;

// trait for drawing triangles
use joeboids::prelude::*;

// progress bar
use indicatif::ProgressIterator;


// This type needs to hold everything required to draw triangles
// for the BoidCanvas trait
struct BoidCanvasWrapper{
    // 2d 'canvas' that we will draw to
    buffer: Vec<Vec<bool>>,
    // for drawing canvas to a file
    writer: png::Writer<BufWriter<std::fs::File>>,
}

impl BoidCanvas for BoidCanvasWrapper{
    // required function, allows the Boid to draw the Boidees it 
    // has calculated
    fn draw_triangle(
            &mut self,
            p1: (f32, f32),
            p2: (f32, f32),
            p3: (f32, f32),
        ) -> Result<(), String> {
            // since we will define boid world dimensions as the same
            // as our image dimensions we can simply round
            let p1 = (p1.0.round() as i32, p1.1.round() as i32);
            let p2 = (p2.0.round() as i32, p2.1.round() as i32);
            let p3 = (p3.0.round() as i32, p3.1.round() as i32);

            // This can be anything, and each Boidee will be drawn 
            // using this function.
            plot_line(&mut self.buffer, p1, p2);
            plot_line(&mut self.buffer, p2, p3);
            plot_line(&mut self.buffer, p3, p1);
            Ok(())
    }
}



pub fn main() {
    // get cli options
    let args = Arguments::parse();
    // `None` means no schedule, which means no pausing to keep to a specific schedule
    let mut flock_master = Boid::new(
        // size of the canvas, tells the boids when to turn around
        // rounded and translated into screen pixels for draw_triangle function
        ((0.0,0.0), (args.width as f32, args.height as f32)),
        None
    );
    if args.tiny{
        flock_master.set_tiny(Some(args.tinyness));
    }
    println!("doing pre-render stepping ...");
    flock_master.init_boidee_random(args.num);
    (0..args.pre_steps).into_iter().progress().for_each(|_|{
        flock_master.raw_step(None);
    });
    println!("Rendering png sequence to {}/ ...", args.output_directory);
    (1..args.frames + 1).into_iter().progress().for_each(|frame_num|{
        let mut canvas = BoidCanvasWrapper{
            buffer: new_buffer_bools(args.width, args.height),
            writer: new_png_writer(format!("{}/{}.png", args.output_directory, frame_num).as_str(), args.width, args.height),
        }; // canvas is new ready to draw to a new file
        // draw to the buffer of bools
        for _ in 0..args.in_betweens{
            flock_master.raw_step(None);
        }
        flock_master.step_on_schedule(&mut canvas, None);
        // draw to the file
        canvas.write().unwrap();
    });
    println!("Done! You can use various software to render your png sequence to a video. Shotcut is a good one.");
}


impl BoidCanvasWrapper{
    // the draw_triangle function used by the Boid modifies the 
    // buffer, this function will submit the buffer to a file
    fn write(&mut self) -> Result<(), png::EncodingError>{
        self.writer.write_image_data(
            // this function take a Vec<u8>. With a bit depth of
            // one, this is 8 pixels per u8. the width/height won't
            // always be divisible by 8, so there will be 
            // buffer zeros (this is what `vec_u8_from_vec_bool` does)
            self.buffer
                .clone()
                .into_iter()
                .map(|image_row|
                    // these are each a row of out image, buffered with
                    // zeros at the ends
                    vec_u8_from_vec_bool(image_row)
                )
                // we currently have a Vec<Vec<u8>>, we want
                // to flatten that to a Vec<u8>
                .flatten()
                .collect::<Vec<u8>>()
                // required for png::encoder::Writer::write_image_data
                .as_slice()
        )
    }
    
}


// CLI arguments
#[derive(Parser)]
#[clap(author="Joseph Peterson", version, about="Joe's crummy boid project")]
struct Arguments{
    #[clap(short='n', long="num", default_value_t=200)]
    /// Number of bird-like-objects to conjure.
    num: usize,
    #[clap(long="width", default_value_t=800)]
    /// Output width.
    width: u32,
    #[clap(long="height", default_value_t=600)]
    /// Output height.
    height: u32,
    #[clap(long="tiny", short='t', default_value_t=false)]
    /// Whether to make the boids tiny. You can specify exactly how tiny with --tinyness.
    tiny: bool,
    #[clap(long="tinyness", default_value_t= 0.5)]
    /// Determines how tiny to make them. Has no effect if -t isn't set. If this is > 1.0 they'll be big.
    tinyness: f32,
    #[clap(long="directory", short='d', default_value_t=String::from("output"))]
    /// Directory put animation frames in. Creates this directory if it isn't present. Existing files will be overwritten (you should probably clear out this directory whenever you decreace the frame count!). Don't use a slash at the end.
    output_directory: String,
    #[clap(long="frames", short='f', default_value_t=300)]
    /// Number of frames to write to the target directory. These will be named in sequence, i.e. 1.png 2.png 3.png etc.
    frames: usize,
    #[clap(long="prestep", short='p', default_value_t=200)]
    pre_steps: u32,
    #[clap(long="inbetweens", short='b', default_value_t=3)]
    in_betweens: u32,
}

// buffer of bools we can 'draw' to
// false represents a dark pixel, true represents a light pixel
fn new_buffer_bools(width: u32, height: u32) -> Vec<Vec<bool>>{
    // for every row
    (0..height).into_iter()
        // map each value (row)
        .map(|_|{
            // which has `width` columns of false
            (0..width).into_iter().map(|_| false)
            // into a Vec<bool>
            .collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>()
}

// Turns a vec of bools into a vec of u8s, buffering with zeros
// at the end
// ex.
// [   t, t, f, f, t, t, f, f,     t, t,]
// [0b_1__1__0__0__1__1__0__0_, 0b_1__1__000000] <- which is [204, 192]
fn vec_u8_from_vec_bool(v: Vec<bool>) -> Vec<u8>{
    let mut out: Vec<u8> = vec![];
    // we go over the array of bools 8 at a time
    let chunked_bits = v.chunks_exact(8);
    // we save any leftovers for later (if v.len() isn't cleanly divisible by 8)
    let remainder = chunked_bits.remainder();
    // bits is an array of bools
    for bits in chunked_bits{
        let mut byte: u8 = 0;
        // iterate over the bools from back to front
        for (i, bit) in bits.into_iter().rev().enumerate(){
            // 2^0, 2^1, ..., 2^7
            // if bit is false we just... don't add it
            byte += *bit as u8 * 2_u8.pow(i as u32);
        }
        out.push(byte);
    }
    // handle the last one, if present
    if remainder.len() > 0 {
        // same thing as above
        let mut last_byte = 0;
        for (i, bit) in remainder.into_iter().rev().enumerate(){
            // we have to offset the power by how many zeros there has to be (so that the data 
            // is on the leftmost of the last byte)
            last_byte += *bit as u8 * 2_u8.pow(i as u32 + (8 - remainder.len()) as u32);
        }
        out.push(last_byte);
    }

    out
}

fn new_png_writer(path: &str,width: u32, height: u32) -> png::Writer<BufWriter<File>>{
    // open a new file or overwrite an existing one with truncate
    let path = std::path::Path::new(path);
    let file = match std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path){
        Ok(v) => v,
        Err(e) => { match e.kind(){
                std::io::ErrorKind::NotFound{..} => {
                    eprintln!("The path `{}` didn't work. Make sure you've created the directory you specify with -d (it defaults to `output`)", path.display());
                    panic!("{e}");
                },
                _ => panic!("{e}"),
            }
        }
    };
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Grayscale);
    // each pixel is a single bit: dark or light, but they still need to be packaged
    // in u8s because... that's the API
    encoder.set_depth(png::BitDepth::One);
    // after writing the header it's ready to write data
    encoder.write_header().unwrap()
}




// This function will be used to draw every boid, the exact details don't really
// matter here (this is just copy/pasted/translated form Wikipedia, it's Bresenham's line algorithm)
fn plot_line(data: &mut Vec<Vec<bool>>, p0: (i32, i32), p1: (i32, i32)){
    if (p1.1 - p0.1).abs() < (p1.0 - p0.0).abs(){
        if p0.0 > p1.0{
            plot_line_low(data, p1, p0);
        }else{
            plot_line_low(data, p0, p1);
        }
    }else{
        if p0.1 > p1.1{
            plot_line_high(data, p1, p0);
        }else{
            plot_line_high(data, p0, p1);
        }
    }
}

fn plot_line_low(data: &mut Vec<Vec<bool>>, p0: (i32, i32), p1: (i32, i32)){
    let mut delta = (p1.0 - p0.0, p1.1 - p0.1);
    let y_change = if delta.1 < 0{
        delta.1 = -delta.1;
        -1
    }else{1};
    let mut d = (2 * delta.1) - delta.0;
    let mut y = p0.1;
    for x in p0.0..p1.0{
        match data.get_mut(y as usize){
            Some(data) =>{
                match data.get_mut(x as usize){
                    // this pixel is light
                    Some(b) => *b = true,
                    None => ()
                }
            },
            None => ()
        }
        if d > 0{
            y = y + y_change;
            d = d + (2 * (delta.1 - delta.0));
        }else{
            d = d + 2 * delta.1;
        }
    }
}

fn plot_line_high(data: &mut Vec<Vec<bool>>, p0: (i32, i32), p1: (i32, i32)){
    let mut delta = (p1.0 - p0.0, p1.1 - p0.1);
    let x_change = if delta.0 < 0{
        delta.0 = -delta.0;
        -1
    }else{1};
    let mut d = (2 * delta.0) - delta.1;
    let mut x = p0.0;
    for y in p0.1..p1.1{
        // data[y as usize][x as usize] = true; <- this would panic  if it's out of range (which happens often
        // with joeboids - there are no out-of-bounds checks in our implementation)
        match data.get_mut(y as usize){
            Some(data) =>{
                match data.get_mut(x as usize){
                    Some(b) => *b = true,
                    None => ()
                }
            },
            None => ()
        }
        if d > 0{
            x = x + x_change;
            d = d + (2 * (delta.0 - delta.1));
        }else{
            d = d + 2 * delta.0;
        }
    }
}
