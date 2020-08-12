use std::path::PathBuf;

use structopt::StructOpt;

/// The rust-life command line configuration.
#[derive(StructOpt, Debug)]
#[structopt(name = "rust-life")]
pub struct Config {
    /// World width.
    #[structopt(short = "w", long = "width", default_value = "80")]
    pub width: u64,

    /// World height.
    #[structopt(short = "h", long = "height", default_value = "48")]
    pub height: u64,

    /// Set generation speed (ms).
    #[structopt(short = "s", long = "speed", default_value = "1000")]
    pub speed: u32,

    /// Input file to load a generation.
    #[structopt(short = "i", long = "input", parse(from_os_str), default_value = "")]
    pub input: PathBuf,

    /// Output file to save a generation.
    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        default_value = "out.rl"
    )]
    pub output: PathBuf,
}
