use std::str::FromStr;

fn evaluate_expression(nums: &[i64], target: i64) -> bool {
    let n = nums.len();
    let mut stack: Vec<(i64, usize)> = vec![(nums[0], 0)];

    while let Some((current_value, index)) = stack.pop() {
        if index == n - 1 {
            if current_value == target {
                return true;
            }
            continue;
        }

        let next_num = nums[index + 1];

        stack.push((current_value + next_num, index + 1));

        stack.push((current_value * next_num, index + 1));
    }

    false
}

fn parse_and_solve_line(line: &str) -> i64 {
    let parts: Vec<&str> = line.split(": ").collect();
    let target: i64 = i64::from_str(parts[0]).unwrap();
    let nums: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|num| i64::from_str(num).unwrap())
        .collect();

    if evaluate_expression(&nums, target) {
        target
    } else {
        0
    }
}

pub fn solve(input: &str) -> i64 {
    input.lines().map(parse_and_solve_line).sum()
}
