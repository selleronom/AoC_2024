use advent_of_code_2024::utils::helpers::get_input;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get day and part from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <day> <part>");
        eprintln!("Example: cargo run 1 1");
        std::process::exit(1);
    }

    let day: u32 = args[1].parse()?;
    let part: u32 = args[2].parse()?;

    let input_data = get_input().await?;
    let result = solve_day(day, part, &input_data)?;
    println!("Result: {}", result);

    Ok(())
}

fn solve_day(day: u32, part: u32, input: &str) -> Result<String, Box<dyn Error>> {
    match (day, part) {
        (1, 1) => Ok(advent_of_code_2024::day01::part1::solve(input).to_string()),
        (1, 2) => Ok(advent_of_code_2024::day01::part2::solve(input).to_string()),
        (2, 1) => Ok(advent_of_code_2024::day02::part1::solve(input).to_string()),
        (2, 2) => Ok(advent_of_code_2024::day02::part2::solve(input).to_string()),
        // Add more days as needed
        _ => Err("Day or part not implemented".into()),
    }
}
