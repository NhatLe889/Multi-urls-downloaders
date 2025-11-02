
// #[tokio::main]
// async fn main() {
//     println!("Hello from Tokio!");
    
//     tokio::spawn(async {
//         println!("Task 1 started");
//         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//         println!("Task 1 finisher");
//     });

//     tokio::spawn(async {
//         println!("Task 2 started");
//         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//         println!("Task 2 finisher");
//     });

//     tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
//     println!("Main function finished");
// }

use core::task;
use std::fs::read_to_string;
use std::process::exit;
use futures::future;

use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_wit_urls>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    SpawnRunTime(&contents);
}

#[tokio::main]
async fn SpawnRunTime(contents: &String) {
    println!("Tokio Starting...");

    let contents_vector: Vec<String> = contents.lines().map(String::from).collect();

    let mut handles = Vec::new();
    for line in contents_vector.into_iter() {
        let handle = tokio::spawn(async move {
            println!("URL: {}", line);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        });
        handles.push(handle);
    }

    let _ = future::join_all(handles).await;
}