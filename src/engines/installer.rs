use rand::distributions::{Alphanumeric, DistString};
use crate::engines::process::is_docker_installed;

fn npm(name: &str, path: &str){
    run(&format!("cd {} && npm install", name), path);
}

fn blazor(name: &str, path: &str){
    run(&format!("dotnet restore {}", name), path);
}


/// Sets up a MongoDB container using Docker with a randomly generated password.
/// The connection string is saved in a .env file for easy access. This is intended for development use only; for production, use a secure method to manage database credentials.
pub fn mongodb(name: &str){
    let mut rng = rand::thread_rng();
    let rand_string: String = Alphanumeric.sample_string(&mut rng, 30);

    println!("Checking if Docker is installed...");
    if !is_docker_installed() {
        eprintln!("❌ Docker is not installed. Please install Docker to use the MongoDB feature.".red());
        return;
    }

    println!("🔐 Generated random password for MongoDB: {}", rand_string);

    run(&format!("docker run --name db-{} -d -p 27017:27017 -e MONGO_INITDB_ROOT_USERNAME=admin -e MONGO_INITDB_ROOT_PASSWORD={} -v mongodb_data:/data/db mongo", name, rand_string ), ".");
    fs::write(format!("{}.env", name), format!("MONGO_URI=mongodb://admin:{}@localhost:27017", rand_string)).expect(|_| (run!("docker rm -f db-{}", name, "."), panic!("Failed to create .env file".red())));
    println!("✅ MongoDB container 'db-{}' created with username 'admin' and password '{}'. The connection string is saved in '{}.env'", name, rand_string, name);
    println!("⚠️ Remember this is a development setup. For production, use a secure method to manage your database credentials.".yellow());
}

fn psql(name: &str){
    
    let mut rng = rand::thread_rng();
    let rand_string: String = Alphanumeric.sample_string(&mut rng, 30);
    
    println!("Checking if Docker is installed...");
    if !is_docker_installed() {
        eprintln!("❌ Docker is not installed. Please install Docker to use the PostgreSQL feature.".red());
        return;
    }

    run(&format!("docker run --name db-{} -d -p 5432:5432 -e POSTGRES_PASSWORD={} -v postgres_data:/var/lib/postgresql/data postgres", name, rand_string), ".");
    fs::write(format!("{}.env", name), format!("POSTGRES_URI=postgres://postgres:{}@localhost:5432", rand_string)).expect(|_| (run!("docker rm -f db-{}", name, "."), panic!("Failed to create .env file".red())));
    println!("✅ PostgreSQL container 'db-{}' created with username 'postgres' and password '{}'. The connection string is saved in '{}.env'", name, rand_string, name);
    println!("⚠️ Remember this is a development setup. For production, use a secure method to manage your database credentials.".yellow());
}
