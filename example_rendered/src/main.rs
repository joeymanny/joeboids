
use clap::Parser;
// use std::thread::sleep;
use joeboids::boid::Boid;
use joeboids::BoidCanvas;

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
    #[clap(long="directory", short='d', default_value_t=String::from("output/"))]
    /// Directory put animation frames in. Creates this directory if it isn't present. Existing files will be overwritten (you should probably clear out this directory whenever you decreace the frame count!)
    output_file: String,
    #[clap(long="frames", short='f', default_value_t=300)]
    /// Number of frames to write to the target directory. These will be named in sequence, i.e. 1.png 2.png 3.png etc.
    frames: usize,
}

pub fn main() {
    
}
