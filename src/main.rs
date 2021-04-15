use chrono::prelude::*;
use git2::{BranchType, Repository};
use std::convert::TryFrom;
use std::io;
use std::io::{Read, Write};

fn main() -> Result<()> {
    let repo = Repository::open_from_env()?;
    let mut stdout = io::stdout();
    let mut stdin = io::stdin().bytes();
    crossterm::terminal::enable_raw_mode()?;
    'branch_loop: for branch in get_branches(&repo, Some(BranchType::Local))? {
        loop {
            write!(
                stdout,
                "\r\n{} ({}) last commit at {} (k/d/q/?) > ",
                branch.name,
                &branch.sha1[0..7],
                branch.time
            )?;
            stdout.flush()?;
            // Unwrapping is okay because stdin.next() never returns None.
            let selection = stdin.next().unwrap()?;
            let action = BranchAction::try_from(selection);
            match action {
                Ok(BranchAction::Keep) => {
                    write!(stdout, "Keeping {}\r\n", branch.name)?;
                }
                Ok(BranchAction::Delete) => {
                    write!(stdout, "Deleting {}\r\n", branch.name)?;
                    unimplemented!();
                }
                Ok(BranchAction::Help) => {
                    write!(stdout, "Help:\r\n")?;
                    write!(stdout, "k - Keep the branch\r\n")?;
                    write!(stdout, "d - Delete the branch\r\n")?;
                    write!(stdout, "q - Quit\r\n")?;
                    write!(stdout, "? - Show this help menu\r\n")?;
                }
                Ok(BranchAction::Quit) => {
                    write!(stdout, "Quitting\r\n")?;
                    break 'branch_loop;
                }
                Err(e) => {
                    write!(stdout, "{}\r\n", e)?;
                }
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn get_branches(repo: &Repository, branch_type: Option<BranchType>) -> Result<Vec<Branch>> {
    let mut branches = repo
        .branches(branch_type)?
        .map(|branch| {
            let (branch, _) = branch?;
            let commit = branch.get().peel_to_commit()?;
            let time = commit.time();
            Ok(Branch {
                name: String::from_utf8_lossy(branch.name_bytes()?).into(),
                sha1: format!("{}", commit.id()),
                time: NaiveDateTime::from_timestamp(
                    time.seconds() + (time.offset_minutes() as i64),
                    0,
                ),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    branches.sort_unstable_by_key(|b| b.time);
    Ok(branches)
}

#[derive(Debug)]
struct Branch {
    name: String,
    sha1: String,
    time: NaiveDateTime,
}

enum BranchAction {
    Keep,
    Delete,
    Quit,
    Help,
}

impl TryFrom<char> for BranchAction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'k' => Ok(BranchAction::Keep),
            'd' => Ok(BranchAction::Delete),
            '?' => Ok(BranchAction::Help),
            'q' => Ok(BranchAction::Quit),
            _ => {
                if value == to_ctrl_char('c') || value == to_ctrl_char('d') {
                    Ok(BranchAction::Quit)
                } else {
                    Err(Error::InvalidInput(value))
                }
            }
        }
    }
}

impl TryFrom<u8> for BranchAction {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        BranchAction::try_from(char::from(value))
    }
}

const fn to_ctrl_char(c: char) -> char {
    (c as u8 & 0b0001_1111) as char
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),
    #[error(transparent)]
    Git2Error(#[from] git2::Error),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("Invalid input `{0}`")]
    InvalidInput(char),
}

type Result<T, E = Error> = std::result::Result<T, E>;
