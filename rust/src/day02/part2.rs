pub fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| is_safe_with_dampener(line))
        .count()
}

fn is_safe_with_dampener(line: &str) -> bool {
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    // If it's already safe, no need to try removing numbers
    if is_safe_report(&levels) {
        return true;
    }

    // Try removing each number once
    for skip_idx in 0..levels.len() {
        let modified: Vec<i32> = levels
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, &x)| x)
            .collect();

        if is_safe_report(&modified) {
            return true;
        }
    }

    false
}

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut increasing: Option<bool> = None;

    for i in 0..levels.len() - 1 {
        let diff = levels[i + 1] - levels[i];

        // Check if difference is within valid range (1-3)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // Determine direction and validate consistency
        if diff > 0 {
            match increasing {
                None => increasing = Some(true),
                Some(false) => return false,
                Some(true) => (),
            }
        } else if diff < 0 {
            match increasing {
                None => increasing = Some(false),
                Some(true) => return false,
                Some(false) => (),
            }
        } else {
            return false; // diff == 0 is not allowed
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(input), 4);
    }
}
