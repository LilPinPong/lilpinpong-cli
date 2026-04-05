pub mod mean;
pub mod stack;

#[derive(Debug, Clone, Copy)]
pub enum Stack {
    Mean,
    Mern,
    Angular,
    React,
    Vue,
    Express,
    MongoDB,
    Blazor,
    DotNetAPIWithAngular,
    DotNetAPIWithReact,
    LaravelWithVue,
}

pub struct StackSource { 
    pub repo_url: &'static str, 
    pub git_ref: &'static str, // tag, branch, or sha
    pub app_dir: &'static str,
    pub server_dir: &'static str,
} 

pub struct ServerSpec {
    pub framework: String,
}

pub struct ProjectSpec {
    pub name: String,
    pub stack: Option<Stack>,
    pub server: Option<ServerSpec>,
    pub yes: bool,
}

pub struct GeneratedFile {
    pub relative_path: String,
    pub contents: String,
}

pub const EXCLUDED_NAMES: &[&str] = &[
    ".git", ".idea", ".vscode", ".angular", "node_modules", "dist", "coverage", "tmp", "out-tsc",
];