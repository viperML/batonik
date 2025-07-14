use tokio::task::spawn_blocking;

use crate::Module;

pub struct Git;

impl Module for Git {
    async fn run(self) -> String {
        let hn = spawn_blocking(|| {
            let repo = match gix::discover(".") {
                Ok(repo) => repo,
                Err(_) => return String::default(),
            };

            repo.head_name().ok().flatten().unwrap().to_string()
        })
        .await
        .unwrap();

        format!("branch: {hn}")
    }
}
