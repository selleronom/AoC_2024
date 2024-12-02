use advent_of_code_2024::utils::helpers::get_input;
use advent_of_code_2024::day02::{part1, part2};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input_data = get_input().await?;

    println!("Part 1: {}", part1::solve(&input_data));
    println!("Part 2: {}", part2::solve(&input_data));

    Ok(())
}
