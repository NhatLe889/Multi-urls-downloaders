
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
use reqwest::{self, Response, header, Client};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use dirs;

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
async fn SpawnRunTime(contents: &String) -> Result<(), Box< dyn std::error::Error>> {
    println!("Tokio Starting...");

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"),
    );
    // "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"

    let contents_vector: Vec<String> = contents.lines().map(String::from).collect();
    let mut handles = Vec::new();

    for line in contents_vector.into_iter() {
        let client = client.clone();
        let headers = headers.clone();
        let url = line.clone();

        let handle = tokio::spawn(async move {
            match client.get(&url).headers(headers).send().await {
                Ok(response) => {
                    // print!("URL: {}", url);
                    // println!("Status: {}", response.status());
                    // println!("Headers:\n{:#?}", response.headers());

                    let specific_dir = dirs::download_dir().unwrap_or(std::env::current_dir().unwrap());

                    let filename = line.split('/').last().unwrap_or("downloaded_file").to_string();
                    println!("Download file: {}", filename);

                    let filepath = specific_dir.join(filename);
                    // println!("{}", filepath.display());
                    let mut file = File::create(&filepath).await.unwrap();
                    let bytes = response.bytes().await.unwrap();
                    file.write_all(&bytes).await.unwrap();


                }
                Err(e) => eprintln!("Request failed for {}: {}", url, e),
            }

        });
        handles.push(handle);
    }

    let _ = future::join_all(handles).await;
    Ok(())

}