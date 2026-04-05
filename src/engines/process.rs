use std::process::Command;

fn is_docker_installed() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
}

fn is_docker_running() -> bool {
    Command::new("docker")
        .arg("info")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .is_ok()
}

fn is_node_installed() -> bool {
    Command::new("node")
        .arg("--version")
        .output()
        .is_ok()
}