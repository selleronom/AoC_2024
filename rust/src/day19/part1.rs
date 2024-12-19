use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();

    let patterns: HashSet<&str> = lines.next().unwrap().split(", ").collect();

    lines.next();

    lines
        .filter(|design| is_possible(design, &patterns))
        .count()
}

fn is_possible(design: &str, patterns: &HashSet<&str>) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            if is_possible(&design[pattern.len()..], patterns) {
                return true;
            }
        }
    }

    false
}
