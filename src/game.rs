use std::{io, thread, time};

use crate::configuration::Config;
use crate::generation::Generation;
use crate::terminal::RawTerminal;

/*
╔════════════════════╗
║───┬───Paused───────║
║ p | pause/unpause  ║
║ s | save           ║
║ q | quit           ║
╚═══╧════════════════╝
*/

/// Represents the game state.
#[allow(dead_code)]
#[derive(PartialEq)]
enum State {
    Paused,
    Running,
}

/// Holds the game data and controls its behavior.
#[allow(dead_code)]
pub struct Game {
    /// The terminal.
    terminal: RawTerminal,

    /// The game state.
    state: State,

    /// The game speed.
    speed: time::Duration,

    /// Current generation.
    current: Generation,

    /// Next generation.
    next: Generation,
}

impl Game {
    /// Create a new Game.
    pub fn new(config: Config, terminal: RawTerminal) -> Self {
        Game {
            terminal,
            state: State::Paused,
            speed: time::Duration::from_millis(u64::from(config.speed)),
            current: Generation::new(config.width, config.height),
            next: Generation::new(config.width, config.height),
        }
    }

    /// Run the game.
    pub fn run(&mut self) -> io::Result<()> {
        self.terminal.clear()?;
        self.terminal.flush()?;

        loop {
            if let Some(input) = self.terminal.read_key() {
                let input = input?;

                if input == b'q' {
                    break;
                }
            }

            self.terminal.clear()?;
            self.draw_generation()?;
            self.terminal.flush()?;

            thread::sleep(self.speed);
        }

        self.terminal.clear()?;
        self.terminal.flush()?;

        Ok(())
    }

    pub fn draw_generation(&mut self) -> io::Result<()> {
        for y in 0..self.current.height {
            for x in 0..self.current.width {
                let cell = self.current.cell(x, y);

                if cell.is_alive() {
                    self.terminal.draw_at('0', x, y)?;
                } else {
                    self.terminal.draw_at(' ', x, y)?;
                }
            }
        }

        Ok(())
    }
}
