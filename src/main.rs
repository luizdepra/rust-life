use std::io;

use structopt::StructOpt;
use termion::async_stdin;
use termion::raw::IntoRawMode;

mod cell;
mod configuration;
mod game;
mod generation;
mod terminal;

use crate::configuration::Config;
use crate::game::Game;
use crate::terminal::Terminal;

fn main() {
    let conf = Config::from_args();

    let stdin = async_stdin();
    let stdout = io::stdout()
        .into_raw_mode()
        .expect("Failed to enter into stdout raw mode");

    let (width, height) = termion::terminal_size().expect("Failed to obtain terminal size");

    let term = Terminal::new(width, height, stdin, stdout);

    let mut game = Game::new(conf, term);
    game.run().expect("Unexpected error when running");
}
