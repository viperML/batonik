use colored::Colorize;

use crate::Module;

pub struct Directory {}

impl Directory {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Module for Directory {
    async fn run(self) -> String {
        let mut cwd = std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let home = std::env::var("HOME").unwrap_or_default();

        if cwd.starts_with(&home) {
            let mut newcwd = String::from("~");
            newcwd.push_str(cwd.trim_start_matches(&home));
            cwd = newcwd;
        }

        return cwd.cyan().bold().to_string();
    }
}
