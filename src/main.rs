use std::fmt;
use std::io;
use std::io::{Read, Write};

fn main() -> Result<(), Error> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin().bytes();
    crossterm::terminal::enable_raw_mode()?;

    loop {
        write!(stdout, ">>> ")?;
        stdout.flush()?;
        let c = stdin.next().unwrap()? as char;
        if c == to_ctrl_byte('c') {
            write!(stdout, "\n\r")?;
            break;
        }
        write!(stdout, "{}\n\r", c)?;
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn to_ctrl_byte(c: char) -> char {
    (c as u8 & 0b0001_1111) as char
}

#[derive(Debug)]
enum Error {
    CrosstermError(crossterm::ErrorKind),
    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CrosstermError(e) => write!(f, "{}", e),
            Error::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::CrosstermError(e) => Some(e),
            Error::IoError(e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<crossterm::ErrorKind> for Error {
    fn from(e: crossterm::ErrorKind) -> Self {
        Error::CrosstermError(e)
    }
}
