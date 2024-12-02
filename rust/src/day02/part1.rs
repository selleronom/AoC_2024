pub fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| is_safe_report(line))
        .count()
}

fn is_safe_report(line: &str) -> bool {
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

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
            return false;  // diff == 0 is not allowed
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
        assert_eq!(solve(input), 2);
    }
}
