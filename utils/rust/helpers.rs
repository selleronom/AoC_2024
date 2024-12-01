use std::env;
use std::fs;
use std::path::PathBuf;
use reqwest;
use std::io::Write;

pub fn extract_day_from_path() -> Result<u32, String> {
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;
    let path = current_dir.to_str().ok_or("Invalid path")?;
    
    path.split(std::path::MAIN_SEPARATOR)
        .find(|segment| segment.starts_with("day"))
        .and_then(|day_segment| day_segment[3..].parse::<u32>().ok())
        .ok_or_else(|| "Unable to extract day from path. Make sure you're running from a 'dayXX' directory.".to_string())
}


pub async fn get_input() -> Result<String, Box<dyn std::error::Error>> {
    let day = extract_day_from_path()?;
    let year = 2024; 
    let session = read_session_cookie()?;
    let input = fetch_or_read_input(year, day, &session).await?;
    Ok(input)
}

fn read_session_cookie() -> Result<String, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !path.join(".aoc_session").exists() {
        if !path.pop() {
            return Err("Could not find .aoc_session file".into());
        }
    }
    let cookie = fs::read_to_string(path.join(".aoc_session"))?;
    Ok(cookie.trim().to_string())
}

async fn fetch_or_read_input(year: u32, day: u32, session: &str) -> Result<String, Box<dyn std::error::Error>> {
    let filename = format!("day_{:02}_input.txt", day);
    if let Ok(input) = fs::read_to_string(&filename) {
        return Ok(input);
    }

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .await?;

    if response.status().is_success() {
        let input = response.text().await?;
        let mut file = fs::File::create(&filename)?;
        file.write_all(input.as_bytes())?;
        Ok(input)
    } else {
        Err(format!("Failed to fetch input: {:?}", response.status()).into())
    }
}
