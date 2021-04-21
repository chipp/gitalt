mod repo;
pub use repo::RepoId;

use git2::{Repository, RepositoryOpenFlags};

pub fn get_current_repo_id() -> Option<RepoId> {
    let repo = match Repository::open_ext(
        ".",
        RepositoryOpenFlags::empty(),
        vec![dirs::home_dir().unwrap()],
    ) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let remotes = repo.remotes().ok()?;

    remotes.iter().find_map(|remote| {
        let remote = repo.find_remote(remote.unwrap()).unwrap();
        RepoId::from_url(remote.url().unwrap())
    })
}

pub fn get_current_branch() -> Option<String> {
    let repo = match Repository::open_ext(
        ".",
        RepositoryOpenFlags::empty(),
        vec![dirs::home_dir().unwrap()],
    ) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let head = repo.head().ok()?;

    if head.is_branch() {
        head.name().map(String::from)
    } else {
        None
    }
}
