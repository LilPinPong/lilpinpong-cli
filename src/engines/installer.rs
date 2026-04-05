use rand::distributions::{Alphanumeric, DistString};

fn angular(name: &str, path: &str){
    run("npm install -g @angular/cli", path);
    run(&format!("ng new {} --directory app --routing --style css", name), path);
    run(&format!("cd {} && npm install", name), path);
}

fn react(name: &str, path: &str){
    run(&format!("npm create vite@latest {} -- --template react", name), path);
    run(&format!("cd {} && npm install", name), path);
}

fn vue(name: &str, path: &str){
    run("npm install -g @vue/cli", path);
    run(&format!("vue create {} --default", name), path);
    run(&format!("cd {} && npm install", name), path);
}

fn express(name: &str, path: &str){
    run("npm init -y", path);
    run("npm install express", path);
}

fn mongodb(name: &str){
    let mut rng = rand::thread_rng();
    let rand_string: String = Alphanumeric.sample_string(&mut rng, 30);

    println!("Checking if Docker is installed...");
    if !is_docker_installed() {
        eprintln!("❌ Docker is not installed. Please install Docker to use the MongoDB feature.");
        return;
    }

    println!("🔐 Generated random password for MongoDB: {}", rand_string);

    run(&format!("docker run --name db-{} -d -p 27017:27017 -e MONGO_INITDB_ROOT_USERNAME=admin -e MONGO_INITDB_ROOT_PASSWORD={} -v mongodb_data:/data/db mongo", name, rand_string ), ".");
    fs::write(format!("{}.env", name), format!("MONGO_URI=mongodb://admin:{}@localhost:27017", rand_string)).expect(|_| (run!("docker rm -f db-{}", name, "."), panic!("Failed to create .env file".red())));
    println!("✅ MongoDB container 'db-{}' created with username 'admin' and password '{}'. The connection string is saved in '{}.env'", name, rand_string, name);
    println!("⚠️ Remember this is a development setup. For production, use a secure method to manage your database credentials.");
}

fn blazor(name: &str, path: &str){
    run(&format!("dotnet new blazorserver -o {}", name), path);
}