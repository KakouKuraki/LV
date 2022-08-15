use std::process::Command;
use std::env;

pub struct Config {
    input_png: String,
    input_wav: String,
    environment_variable: Environment,
    output_mp4: String,
}

enum Environment {
    HighQuarity,
    Twitter,
}

pub fn run(config: Config) {
    match config.environment_variable {
        Environment::HighQuarity => {
            let mut process = Command::new("ffmpeg")
            .arg("-framerate")
            .arg("30")
            .arg("-i")
            .arg(&config.input_png)
            .arg("-i")
            .arg(&config.input_wav)
            .arg("-vcodec")
            .arg("libx264")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("-crf")
            .arg("18")
            .arg(&config.output_mp4)
            .spawn().expect("cannot execute ffmpeg");
            process.wait();
        },
        Environment::Twitter => {
            let mut process = Command::new("ffmpeg")
            .arg("-framerate")
            .arg("30")
            .arg("-i")
            .arg(&config.input_png)
            .arg("-i")
            .arg(&config.input_wav)
            .arg("-vcodec")
            .arg("libx264")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("-s")
            .arg("1280x720")
            .arg(&config.output_mp4)
            .spawn().expect("cannot execute ffmpeg");
            process.wait();
            println!("LOW!!!!!!!!!!!!!!!!!!!!");
        },
    };
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let input_png = match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot get a input png string"),
        };

        let input_wav = match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot get a input wav string"),
        };

        let output_mp4 = match args.next() {
            Some(arg) => arg,
            None => return Err("Cannot get a output mp4 string"),
        };

        let is_high_quarity = env::var("HIGH_QUARITY").is_ok();

        if is_high_quarity {
            Ok(Config {input_png, input_wav, environment_variable: Environment::HighQuarity, output_mp4})
        } else {
            Ok(Config {input_png, input_wav, environment_variable: Environment::Twitter, output_mp4})
        }
    }
}
