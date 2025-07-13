use colored::Colorize;

use crate::Module;

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
        let v = std::env::var(self.var).ok();

        if let Some(v) = v {
            return (self.style)(v);
        } else {
            return String::default();
        }
    }
}