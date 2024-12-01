use std::collections::HashMap;
use aoc_utils::helpers::get_input;
use std::error::Error;

// Part1
fn calculate_total_distance(input_data: &str) -> i32 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Split the input into left and right lists
    for line in input_data.lines() {
        let mut numbers = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        if let (Some(left), Some(right)) = (numbers.next(), numbers.next()) {
            left_list.push(left);
            right_list.push(right);
        }
    }

    // Sort both lists
    left_list.sort_unstable();
    right_list.sort_unstable();

    // Calculate the total distance
    left_list.iter().zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

//Part 2
fn calculate_similarity_score(input_data: &str) -> i32 {
    let mut left_list = Vec::new();
    let mut right_count = HashMap::new();

    // Parse input and populate left_list and right_count
    for line in input_data.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            *right_count.entry(numbers[1]).or_insert(0) += 1;
        }
    }

    // Calculate similarity score
    left_list.iter()
        .map(|&num| num * right_count.get(&num).unwrap_or(&0))
        .sum()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input_data = get_input().await?;

    let result = calculate_total_distance(&input_data);
    println!("The total distance between the lists is: {}", result);

    let result = calculate_similarity_score(&input_data);
    println!("The similarity score is: {}", result);

    Ok(())
}
