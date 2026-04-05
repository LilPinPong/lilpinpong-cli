use crate::stacks::StackSource;

pub fn source() -> StackSource {
    StackSource {
        repo_url: "https://github.com/LilPinPong/tbd-2-react",
        git_ref: "main",
        app_dir: "app",
        server_dir: "server",
    }
}