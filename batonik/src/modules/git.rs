use crate::Module;

pub struct Git;

impl Module for Git {
    async fn run(self) -> String {
        return String::from("branch: master");
    }
}
