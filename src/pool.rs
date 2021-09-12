extern crate serde;
extern crate fs_extra;
use fs_extra::file::CopyOptions;
use fs_extra::file::copy;

use std::fs;
use std::time::Instant;
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::sync::mpsc::channel;
use workerpool::Pool;
use workerpool::thunk::{Thunk, ThunkWorker};

#[derive(Serialize, Deserialize, Debug)]
struct CopyFile {
  src: String,
  dest: String
}

fn main() -> std::io::Result<()> {
    let n_workers = 4;
    let pool = Pool::<ThunkWorker<()>>::new(n_workers);
    println!("Creating {} threads", pool.max_count());

    let json_file_path = Path::new("./queue.json");
    let file = File::open(json_file_path).expect("file not found");
    let files:Vec<CopyFile> = serde_json::from_reader(file).expect("file not found");

    println!("Hello, world!");
    println!("Copying {} files", files.len());

    let (tx, _rx) = channel();
    let now = Instant::now();
    for file in files {
        pool.execute_to(tx.clone(), Thunk::of(|| {
            let parent = Path::new(&file.dest).parent().unwrap();
            let mut options = CopyOptions::new();
            options.overwrite = true;
        
            // println!("Copy {} to {}", file.src, parent.to_string_lossy());
            fs::create_dir_all(parent).expect("Could not create directory");
            copy(file.src, file.dest, &options).expect("Failed to copy");  // Copy foo.txt to bar.txt
        }));
    }

    pool.join();

    println!("Took: {:.2?}", now.elapsed());

    Ok(())
}
