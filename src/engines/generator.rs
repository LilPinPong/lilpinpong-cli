use std::fs;
use std::path::Path;


pub fn generate_project_directory(name: &str, backend: &bool) {
    let root = Path::new(name);

    if root.exists() {  
        panic!("❌ Folder already exists");
    }

    fs::create_dir(root).expect("Failed to create project directory");
    fs::create_dir(root.join("app")).expect("Failed to create app directory");
    if *backend {
        fs::create_dir(root.join("server")).expect("Failed to create server directory");
    }
    fs::write(root.join("README.md"), format!("# {}", name)).expect("Failed to create README.md".red());
    fs::write(root.join(".gitignore"), ".env*\n").expect("Failed to create .gitignore".red());

    println!(" ✅ Generated project directory '{}'", name);
}