use std::{io, mem, thread, time};

use itertools::iproduct;
use liblife::cell::{Cell, SimpleCell};
use liblife::generation::{Generation, SimpleGeneration};
use liblife::result::{Error, Result};

use crate::configuration::Config;
use crate::terminal::{Color, RawTerminal};

const BLOCK_CHAR: char = '▀';

/// Represents the game state.
#[allow(dead_code)]
#[derive(PartialEq)]
enum State {
    Paused,
    Running,
}

/// Holds the game data and controls its behavior.
#[allow(dead_code)]
pub struct Game<T>
where
    T: Cell,
{
    /// The terminal.
    terminal: RawTerminal,

    /// The game state.
    state: State,

    /// The game speed.
    speed: time::Duration,

    /// Current generation.
    current: Generation<T>,

    /// Next generation.
    next: Generation<T>,
}

impl<T> Game<T>
where
    T: Cell
{
    /// Create a new Game.
    pub fn new(config: Config, terminal: RawTerminal) -> Result<Self> {
        Ok(Game {
            terminal,
            state: State::Paused,
            speed: time::Duration::from_millis(u64::from(config.speed)),
            current: Generation::new(config.width, config.height)?,
            next: Generation::new(config.width, config.height)?,
        })
    }

    /// Run the game.
    pub fn run(&mut self) -> io::Result<()> {
        self.init_random();

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

            self.next_generation();
        }

        self.terminal.clear()?;
        self.terminal.flush()?;

        Ok(())
    }

    pub fn draw_generation(&mut self) -> io::Result<()> {
        for y in (0..self.current.height).step_by(2) {
            for x in 0..self.current.width {
                let top_cell = self.current.cell(x, y);
                let bottom_cell = self.current.cell(x, y + 1);

                let (fg, bg) = match (top_cell.is_alive(), bottom_cell.is_alive()) {
                    (true, true) => (Color::White, Color::White),
                    (true, false) => (Color::White, Color::Black),
                    (false, true) => (Color::Black, Color::White),
                    (false, false) => (Color::Black, Color::Black),
                };

                self.terminal.set_foreground(fg)?;
                self.terminal.set_background(bg)?;
                self.terminal.draw_at(BLOCK_CHAR, x, y / 2)?;
                self.terminal.reset_styling()?;
            }
        }

        Ok(())
    }

    fn next_generation(&mut self) {
        // TODO verify generation sizes
        iproduct!(0..self.current.height, 0..self.current.width).for_each(|(y, x)| {
            let target_cell = self.next.mut_cell(x, y);
            if Ruleset::should_live(x, y, &self.current) {
                target_cell.spawn();
            } else {
                target_cell.kill();
            }
        });

        mem::swap(&mut self.current, &mut self.next);
    }

    fn init_random(&mut self) {
        for y in 0..self.current.height {
            for x in 0..self.current.width {
                let cell = self.current.mut_cell(x, y);
                if rand::random() {
                    cell.spawn();
                } else {
                    cell.kill();
                }
            }
        }
    }
}
