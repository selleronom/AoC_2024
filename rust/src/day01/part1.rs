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
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn solve(input: &str) -> String {
    let result = calculate_total_distance(input);
    result.to_string()
}
