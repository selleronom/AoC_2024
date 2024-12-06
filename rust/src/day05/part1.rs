use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> i32 {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    let rules: Vec<(&str, &str)> = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            (parts[0], parts[1])
        })
        .collect();

    let updates: Vec<Vec<&str>> = sections[1]
        .lines()
        .map(|line| line.split(',').collect())
        .collect();

    let mut precedence: HashMap<&str, Vec<&str>> = HashMap::new();

    for (x, y) in rules {
        precedence.entry(x).or_insert_with(Vec::new).push(y);
    }

    let mut sum_of_middles = 0;

    'outer: for update in updates {
        let set: HashSet<&str> = update.iter().cloned().collect();
        let mut order_map: HashMap<&str, usize> = HashMap::new();

        for (i, &page) in update.iter().enumerate() {
            order_map.insert(page, i);
        }

        for (x, ys) in &precedence {
            if set.contains(x) {
                for &y in ys {
                    if set.contains(y) && order_map[x] >= order_map[y] {
                        continue 'outer;
                    }
                }
            }
        }

        let middle_index = update.len() / 2;
        sum_of_middles += update[middle_index].parse::<i32>().unwrap();
    }

    sum_of_middles
}
