use chrono::prelude::*;
use git2::{BranchType, Repository};

fn main() -> Result<()> {
    let repo = git2::Repository::open_from_env()?;

    let local_branches = list_branches(&repo, Some(BranchType::Local))?;
    let remote_branches = list_branches(&repo, Some(BranchType::Remote))?;
    println!("Local branches:");
    println!("{}", local_branches);
    println!("Remote branches:");
    println!("{}", remote_branches);

    Ok(())
}

fn get_branches(repo: &Repository, branch_type: Option<BranchType>) -> Result<Vec<Branch>> {
    repo.branches(branch_type)?
        .map(|branch| {
            let (branch, _) = branch?;
            let commit = branch.get().peel_to_commit()?;
            let time = commit.time();
            Ok(Branch {
                name: String::from_utf8_lossy(branch.name_bytes()?).into(),
                commit_id: format!("{}", commit.id()),
                time: NaiveDateTime::from_timestamp(
                    time.seconds() + (time.offset_minutes() as i64),
                    0,
                ),
            })
        })
        .collect()
}

fn list_branches(repo: &Repository, branch_type: Option<BranchType>) -> Result<String> {
    let branches = get_branches(repo, branch_type)?
        .into_iter()
        .map(|branch| Ok(format!("{} ({})\n", branch.name, &branch.commit_id[0..7])))
        .collect::<Result<Vec<_>>>()?;
    Ok(branches.join(""))
}

#[derive(Debug)]
struct Branch {
    name: String,
    commit_id: String,
    time: NaiveDateTime,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),
    #[error(transparent)]
    Git2Error(#[from] git2::Error),
}

type Result<T, E = Error> = std::result::Result<T, E>;
