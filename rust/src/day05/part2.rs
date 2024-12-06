//part2.rs
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    // Parse rules until empty line
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<i32> = line.split('|').map(|x| x.parse().unwrap()).collect();
        rules.entry(parts[0]).or_default().insert(parts[1]);
    }

    let mut sum = 0;
    while let Some(line) = lines.next() {
        let pages: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();

        if !is_valid_order(&pages, &rules) {
            let mut ordered = pages.clone();
            order_pages(&mut ordered, &rules);
            sum += ordered[ordered.len() / 2];
        }
    }

    sum
}

fn is_valid_order(pages: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for (i, &page) in pages.iter().enumerate() {
        if let Some(must_follow) = rules.get(&page) {
            for &follow in must_follow {
                if pages[i + 1..].contains(&follow) {
                    continue;
                }
                if pages[..i].contains(&follow) {
                    return false;
                }
            }
        }
    }
    true
}

fn order_pages(pages: &mut [i32], rules: &HashMap<i32, HashSet<i32>>) {
    let n = pages.len();
    for _i in 0..n {
        for j in 0..n - 1 {
            if let Some(follows) = rules.get(&pages[j]) {
                if follows.contains(&pages[j + 1]) {
                    pages.swap(j, j + 1);
                }
            }
        }
    }
}
