fn main() -> Result<()> {
    let repo = git2::Repository::open_from_env()?;

    let local_branches = list_branches(&repo, git2::BranchType::Local)?;
    let remote_branches = list_branches(&repo, git2::BranchType::Remote)?;
    println!("Local branches:");
    println!("{}", local_branches);
    println!("Remote branches:");
    println!("{}", remote_branches);

    Ok(())
}

fn list_branches(repo: &git2::Repository, branch_type: git2::BranchType) -> Result<String> {
    let mut branches = String::new();
    for branch in repo.branches(Some(branch_type))? {
        let (branch, _) = branch?;
        let name = String::from_utf8_lossy(branch.name_bytes()?);
        let commit_oid = branch.get().peel_to_commit()?.id();
        let commit_sha1 = format!("{}", commit_oid);
        branches.push_str(&format!("{} ({})\n", name, &commit_sha1[0..7]));
    }
    Ok(branches)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),
    #[error(transparent)]
    Git2Error(#[from] git2::Error),
}

type Result<T, E = Error> = std::result::Result<T, E>;
