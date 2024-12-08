pub fn solve(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let test_value: i64 = parts[0].parse().unwrap();
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if can_form_value_with_concat(test_value, &numbers) {
            total += test_value;
        }
    }
    total
}

fn can_form_value_with_concat(target: i64, numbers: &[i64]) -> bool {
    let len = numbers.len();
    // Use 3^(len-1) to generate all possible combinations of 3 operators
    let max_ops = 3_i64.pow((len - 1) as u32);

    for ops in 0..max_ops {
        let mut result = numbers[0];
        let mut curr_ops = ops;

        for i in 1..len {
            let op = curr_ops % 3;
            curr_ops /= 3;

            match op {
                0 => result += numbers[i],
                1 => result *= numbers[i],
                2 => result = concatenate(result, numbers[i]),
                _ => unreachable!(),
            }
        }

        if result == target {
            return true;
        }
    }
    false
}

fn concatenate(left: i64, right: i64) -> i64 {
    let left_str = left.to_string();
    let right_str = right.to_string();
    (left_str + &right_str).parse().unwrap()
}
