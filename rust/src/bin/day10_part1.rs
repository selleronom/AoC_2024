use advent_of_code_2024::day10::part1;
use advent_of_code_2024::utils::helpers::get_input;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input_data = get_input().await?;

    println!("{}", part1::solve(&input_data));

    Ok(())
}
