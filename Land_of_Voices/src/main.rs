extern crate ffmpeg_wrapper;
use std::env;

fn main() {
    let config = ffmpeg_wrapper::Config::new(env::args()).unwrap();
    ffmpeg_wrapper::run(config);
}
