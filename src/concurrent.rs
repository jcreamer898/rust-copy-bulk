use fs_extra::file::CopyOptions;
use fs_extra::file::copy;
use futures::{stream, StreamExt};
use std::time::Instant;
use std::fs;
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct CopyFile {
  src: String,
  dest: String
}

async fn compute_job(file: CopyFile) -> () {
    let parent = Path::new(&file.dest).parent().unwrap();
    let mut options = CopyOptions::new();
    options.overwrite = true;

    // println!("Copy {} to {}", file.src, parent.to_string_lossy());
    fs::create_dir_all(parent).expect("Could not create directory");
    copy(file.src, file.dest, &options).expect("Failed to copy");
}

#[tokio::main]
async fn main() {
    println!("Reading input...");

    let json_now = Instant::now();
    let json_file_path = Path::new("./queue.json");
    let file = File::open(json_file_path).expect("file not found");
    let files:Vec<CopyFile> = serde_json::from_reader(file).expect("file not found");

    println!("Took: {:.2?} to parse JSON", json_now.elapsed());
    println!("Copying {} files", files.len());

    let concurrency = 100;

    let now = Instant::now();
    stream::iter(files)
        .for_each_concurrent(concurrency, |job| async move {
            compute_job(job).await;
        })
        .await;
    println!("Took: {:.2?}", now.elapsed());
}