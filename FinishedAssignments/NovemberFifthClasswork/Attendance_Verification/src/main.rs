use serde::Deserialize;
use std::{error::Error, fmt, fs::File, io::Write, path::Path};

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    Error(AppError),
}

#[derive(Debug)]
enum AppError {
    HttpError(u16),
    JsonParseError(String),
    NetworkError(String),
    IoError(String),
    DownloadError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::HttpError(code) => write!(f, "HTTP error with status code {}", code),
            AppError::JsonParseError(e) => write!(f, "Failed to parse JSON: {}", e),
            AppError::NetworkError(e) => write!(f, "Network request failed: {}", e),
            AppError::IoError(e) => write!(f, "I/O error: {}", e),
            AppError::DownloadError(e) => write!(f, "Failed to download image: {}", e),
        }
    }
}

impl Error for AppError {}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";

    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::Error(AppError::JsonParseError(e.to_string())),
                }
            } else {
                ApiResult::Error(AppError::HttpError(response.status()))
            }
        }
        Err(e) => ApiResult::Error(AppError::NetworkError(e.to_string())),
    }
}


fn download_image(url: &str, filename: &str) -> Result<(), AppError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| AppError::NetworkError(e.to_string()))?;

    if response.status() != 200 {
        return Err(AppError::HttpError(response.status()));
    }

    let mut reader = response.into_reader();
    let mut buffer = Vec::new();
    std::io::copy(&mut reader, &mut buffer)
        .map_err(|e| AppError::DownloadError(e.to_string()))?;

    let path = Path::new(filename);
    let mut file = File::create(path).map_err(|e| AppError::IoError(e.to_string()))?;
    file.write_all(&buffer)
        .map_err(|e| AppError::IoError(e.to_string()))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);

                let filename = format!("dog_image_{}.jpg", i);
                match download_image(&dog_image.message, &filename) {
                    Ok(_) => println!("💾 Saved as '{}'", filename),
                    Err(e) => println!("❌ Failed to save image: {}", e),
                }

                println!("📊 Status: {}", dog_image.status);
            }
            ApiResult::Error(e) => println!("❌ Error: {}", e),
        }
        println!();
    }

    Ok(())
}
