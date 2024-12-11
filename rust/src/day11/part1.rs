use std::collections::VecDeque;

pub fn solve(input: &str) -> usize {
    let mut stones: VecDeque<u32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut next_stones = VecDeque::new();
        while let Some(stone) = stones.pop_front() {
            if stone == 0 {
                next_stones.push_back(1);
            } else if stone.to_string().len() % 2 == 0 {
                let digits = stone.to_string();
                let mid = digits.len() / 2;
                let left = digits[..mid].parse().unwrap();
                let right = digits[mid..].parse().unwrap();
                next_stones.push_back(left);
                next_stones.push_back(right);
            } else {
                next_stones.push_back(stone * 2024);
            }
        }
        stones = next_stones;
    }

    stones.len()
}
