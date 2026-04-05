use crate::stacks::StackSource;

pub fn source() -> StackSource {
    StackSource {
        repo_url: "https://github.com/LilPinPong/mean.git",
        git_ref: "main",
        app_dir: "app",
        server_dir: "server",
    }
}