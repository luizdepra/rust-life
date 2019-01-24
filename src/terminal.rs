use std::io::{self, Read, Write};

use termion::{self, cursor, raw};

/// A terminal abstraction with input, output and events handling.
#[derive(Debug)]
pub struct Terminal<R, W>
where
    R: Read,
    W: Write,
{
    /// Screen width.
    pub width: u16,

    /// Screen height.
    pub height: u16,

    /// Stdin handler.
    stdin: io::Bytes<R>,

    /// Stdout handler.
    stdout: W,
}

/// RawTerminal is a type defined with AsyncReader and RawTerminal.
pub type RawTerminal = Terminal<termion::AsyncReader, raw::RawTerminal<io::Stdout>>;

impl<R, W> Terminal<R, W>
where
    R: Read,
    W: Write,
{
    /// Creates a new Terminal.
    pub fn new(width: u16, height: u16, stdin: R, stdout: W) -> Self {
        Terminal {
            width,
            height,
            stdin: stdin.bytes(),
            stdout,
        }
    }

    pub fn read_key(&mut self) -> Option<Result<u8, io::Error>> {
        self.stdin.next()
    }

    pub fn clear(&mut self) -> io::Result<()> {
        write!(
            self.stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    pub fn draw_at(&mut self, symbol: char, x: u16, y: u16) -> io::Result<()> {
        // Goto is one-based, so we need to add 1 to each coord.
        let position = cursor::Goto(x + 1, y + 1);

        write!(self.stdout, "{}{}", position, symbol)
    }
}
