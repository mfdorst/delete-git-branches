use chrono::prelude::*;
use git2::{BranchType, Repository};
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
            let selection = stdin.next().unwrap()? as char;
            match selection {
                'k' => {
                    write!(stdout, "Keeping {}\r\n", branch.name)?;
                    break;
                }
                'd' => {
                    write!(stdout, "Deleting {}\r\n", branch.name)?;
                    // TODO: Implement delete
                    break;
                }
                '?' => {
                    write!(stdout, "Help:\r\n")?;
                    write!(stdout, "k - Keep the branch\r\n")?;
                    write!(stdout, "d - Delete the branch\r\n")?;
                    write!(stdout, "q - Quit\r\n")?;
                    write!(stdout, "? - Show this help menu\r\n")?;
                }
                '\r' => {
                    write!(
                        stdout,
                        "Please select an option. Press '?' for help or 'q' to quit."
                    )?;
                }
                _ => {
                    if selection == 'q'
                        || selection == to_ctrl_char('c')
                        || selection == to_ctrl_char('d')
                    {
                        write!(stdout, "Quitting\r\n")?;
                        break 'branch_loop;
                    } else {
                        write!(
                            stdout,
                            "Invalid selection '{}'. Type '?' for help or 'q' to quit.",
                            selection
                        )?;
                    }
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
                sha1: commit.id().to_string(),
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
}

type Result<T, E = Error> = std::result::Result<T, E>;
