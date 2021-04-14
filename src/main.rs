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

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),
    #[error(transparent)]
    IoError(#[from] io::Error),
}
