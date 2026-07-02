use std::fs::File;
use std::io::Write;
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReturnDownloadResult {
    pub status: bool,
    pub message: String,
}

pub async fn new(url: &str, output_file: &str) -> ReturnDownloadResult {
    let client = Client::new();

    let response = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            return ReturnDownloadResult {
                status: false,
                message: format!("Request failed: {}", e),
            };
        }
    };

    let total_size = match response.content_length() {
        Some(len) => len,
        None => {
            return ReturnDownloadResult {
                status: false,
                message: "Couldn't determine file size.".to_string(),
            };
        }
    };

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let path = Path::new(output_file);
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            return ReturnDownloadResult {
                status: false,
                message: format!("Failed to create file: {}", e),
            };
        }
    };

    let mut response = response;
    let mut downloaded: u64 = 0;

    loop {
        let chunk = match response.chunk().await {
            Ok(Some(bytes)) => bytes,
            Ok(None) => break,
            Err(e) => {
                return ReturnDownloadResult {
                    status: false,
                    message: format!("Read error: {}", e),
                };
            }
        };

        if let Err(e) = file.write_all(&chunk) {
            return ReturnDownloadResult {
                status: false,
                message: format!("Write error: {}", e),
            };
        }

        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("✅ Download complete");

    ReturnDownloadResult {
        status: true,
        message: format!("Saved to {}", output_file),
    }
}