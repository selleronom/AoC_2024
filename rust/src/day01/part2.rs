use std::collections::HashMap;

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

pub fn solve(input: &str) -> String {
    let result = calculate_similarity_score(input);
    result.to_string()
}