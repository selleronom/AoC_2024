use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> i32 {
    let corrupted = parse_input(input);
    find_shortest_path(&corrupted)
}

fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .take(1024)
        .map(|line| {
            let mut coords = line.split(',');
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn find_shortest_path(corrupted: &HashSet<(i32, i32)>) -> i32 {
    let target = (70, 70);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(((0, 0), 0));
    visited.insert((0, 0));

    while let Some((pos, steps)) = queue.pop_front() {
        if pos == target {
            return steps;
        }

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let next = (pos.0 + dx, pos.1 + dy);

            if next.0 >= 0
                && next.0 <= 70
                && next.1 >= 0
                && next.1 <= 70
                && !corrupted.contains(&next)
                && !visited.contains(&next)
            {
                visited.insert(next);
                queue.push_back((next, steps + 1));
            }
        }
    }

    -1 // No path found
}
