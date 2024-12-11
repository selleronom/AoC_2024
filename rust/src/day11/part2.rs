use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let initial_stones: Vec<BigUint> = input
        .split_whitespace()
        .map(|s| s.parse::<BigUint>().unwrap())
        .collect();

    let mut memo = HashMap::new();
    let mut total = 0;

    for stone in initial_stones {
        total += count_stones(&stone, 75, &mut memo);
    }

    total
}

fn count_stones(n: &BigUint, k: usize, memo: &mut HashMap<(BigUint, usize), usize>) -> usize {
    if k == 0 {
        return 1;
    }
    if let Some(&res) = memo.get(&(n.clone(), k)) {
        return res;
    }
    let next_stones = apply_rules(n);
    let mut total = 0;
    for stone in next_stones {
        total += count_stones(&stone, k - 1, memo);
    }
    memo.insert((n.clone(), k), total);
    total
}

fn apply_rules(n: &BigUint) -> Vec<BigUint> {
    if n.is_zero() {
        vec![BigUint::one()]
    } else {
        let n_str = n.to_str_radix(10);
        if n_str.len() % 2 == 0 {
            let mid = n_str.len() / 2;
            let left = n_str[..mid].trim_start_matches('0');
            let right = n_str[mid..].trim_start_matches('0');
            let mut stones = Vec::new();
            if !left.is_empty() {
                stones.push(BigUint::parse_bytes(left.as_bytes(), 10).unwrap());
            } else {
                stones.push(BigUint::zero());
            }
            if !right.is_empty() {
                stones.push(BigUint::parse_bytes(right.as_bytes(), 10).unwrap());
            } else {
                stones.push(BigUint::zero());
            }
            stones
        } else {
            vec![n * BigUint::from(2024u32)]
        }
    }
}
