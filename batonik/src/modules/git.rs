use gix::progress::Discard;
use tokio::task::spawn_blocking;

use crate::Module;

pub struct Git;

impl Module for Git {
    async fn run(self) -> String {
        let repo = match gix::discover(".") {
            Ok(repo) => repo,
            Err(_) => return String::default(),
        };

        let branch = repo.head_name().ok().flatten().unwrap().to_string();

        let status = repo.status(Discard).unwrap().index_worktree_options_mut(cb);


        format!("branch: {branch}")
    }
}
