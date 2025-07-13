use std::env;

use colored::Colorize;

use crate::Module;

pub struct Git;

impl Module for Git {
    async fn run(self) -> String {
        return String::from("branch: master");
    }
}

pub struct Env {
    var: String,
    style: fn(String) -> String,
}

impl Env {
    pub fn default_style(s: String) -> String {
        return format!("with {}", s.white().dimmed());
    }

    pub fn new(var: impl ToString) -> Self {
        Self {
            var: var.to_string(),
            style: Self::default_style,
        }
    }
}

impl Module for Env {
    async fn run(self) -> String {
        let v = env::var(self.var).ok();

        if let Some(v) = v {
            return (self.style)(v);
        } else {
            return String::default();
        }
    }
}

pub struct Directory {}

impl Directory {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Module for Directory {
    async fn run(self) -> String {
        let mut cwd = env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let home = env::var("HOME").unwrap_or_default();

        if cwd.starts_with(&home) {
            let mut newcwd = String::from("~");
            newcwd.push_str(cwd.trim_start_matches(&home));
            cwd = newcwd;
        }

        return cwd.cyan().bold().to_string();
    }
}
