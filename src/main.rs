use std::path::Path;
use glob::glob;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
  dir: String,
  package_json: String
}

fn main() {
    let cwd = env::current_dir();
    let pkgs: Vec<&Workspace> = vec![];

    println!("Reading input...");
    println!("{:?}", cwd);
    
    let pkgGlob = Path::new(&cwd);
  
    for entry in glob(pkgGlob).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", Path::new(&cwd).join(path)),
            Err(e) => println!("{:?}", e),
        }
    }
}