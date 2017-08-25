extern crate rodio;

use rodio::buffer::SamplesBuffer;
use std::io;
mod sine;

fn main() {


    let endpoint = rodio::get_default_endpoint().unwrap();

    let source = sine::SineWave::new(0f32,std::time::Duration::new(1,0));
    rodio::play_raw(&endpoint, source);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
}
