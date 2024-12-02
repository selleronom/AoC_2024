use reqwest;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn extract_day_from_filename() -> Result<u32, String> {
    let args: Vec<String> = env::args().collect();
    let exe_name = args.get(0).ok_or("Unable to get executable name")?;
    let path = Path::new(exe_name);
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Unable to get filename")?;

    if filename.starts_with("day") {
        filename[3..]
            .parse::<u32>()
            .map_err(|_| "Invalid day number in filename".to_string())
    } else {
        Err("This is not a day executable".to_string())
    }
}

pub async fn get_input() -> Result<String, Box<dyn std::error::Error>> {
    let day = extract_day_from_filename()?;
    let year = 2024;
    let session = read_session_cookie()?;
    let input = fetch_or_read_input(year, day, &session).await?;
    Ok(input)
}

fn read_session_cookie() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(cookie) = env::var("AOC_SESSION") {
        return Ok(cookie.trim().to_string());
    }

    let env_path = Path::new(".env");
    if env_path.exists() {
        let contents = fs::read_to_string(env_path)?;
        for line in contents.lines() {
            if let Some(cookie) = line.strip_prefix("AOC_SESSION=") {
                return Ok(cookie.trim().to_string());
            }
        }
    }

    Err("AOC_SESSION not found in environment or .env file".into())
}

async fn fetch_or_read_input(
    year: u32,
    day: u32,
    session: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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
