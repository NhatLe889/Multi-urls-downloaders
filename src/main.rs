
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
use std::fmt::format;
use std::fs::read_to_string;
use std::process::exit;
use futures::future;
use reqwest::{self, Response, header, Client};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use dirs;
use std::{env, fs, process};
use futures::stream::StreamExt;
use indicatif::{MultiProgress, ProgressStyle, ProgressBar};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_wit_urls>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e)=> {
            eprintln!("Failed to read file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    if let Err(e) = SpawnRunTime(&contents).await {
        eprintln!("\nAn error occur during progress: {}", e);
        process::exit(1);
    }
}

async fn process_url(client: reqwest::Client, headers: HeaderMap, url: &str, progress_bar: ProgressBar,) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).headers(headers).send().await?;

    if !response.status().is_success() {
        let error_msg = format!("Request failed with the following status: {}", response.status());
        progress_bar.finish_with_message(error_msg.clone());
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_msg)));
    }

    let total_size = response.content_length().unwrap_or(0);

    if total_size > 0 {
        progress_bar.set_style(ProgressStyle::with_template("{Spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")?
            .progress_chars("#>-"));
        progress_bar.set_length(total_size);
    } else {
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {bytes} downloaded {msg}")?);
    }


    let download_dir = dirs::download_dir().ok_or("Downloads directory could not be found")?;
    let filename = url.split('/').last().unwrap_or("download_file");
    let filepath = download_dir.join(filename);
    let mut file = File::create(&filepath).await?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        progress_bar.set_position(downloaded);
    }

    progress_bar.finish_with_message(format!("Downloaded to {}", filepath.display()));

    Ok(())
}

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

    let multi_progress = MultiProgress::new();

    for url in contents_vector {
        let client = client.clone();
        let headers = headers.clone();

        let progress_bar = multi_progress.add(ProgressBar::new(0));
        let filename = url.split('/').last().unwrap_or("").to_string();
        progress_bar.set_message(filename);

        let handle = tokio::spawn(async move {
            if let Err(e) = process_url(client, headers, &url, progress_bar).await {
                eprintln!("\nAn error occur while downloading {}: {}", url, e);
            }
        });
        handles.push(handle);
    }

    future::join_all(handles).await;

    Ok(())
}