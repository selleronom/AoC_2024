use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> String {
let all_coords: Vec<(i32, i32)> = input
.lines()
.map(|line| {
let mut coords = line.split(',');
let x = coords.next().unwrap().parse().unwrap();
let y = coords.next().unwrap().parse().unwrap();
(x, y)
})
.collect();

for i in 0..all_coords.len() {
let corrupted: HashSet<(i32, i32)> = all_coords[0..=i].iter().cloned().collect();

if !has_path_to_exit(&corrupted) {
let blocking_coord = all_coords[i];
return format!("{},{}", blocking_coord.0, blocking_coord.1);
}
}

"No solution found".to_string()
}

fn has_path_to_exit(corrupted: &HashSet<(i32, i32)>) -> bool {
let target = (70, 70);
let mut queue = VecDeque::new();
let mut visited = HashSet::new();

queue.push_back((0, 0));
visited.insert((0, 0));

while let Some(pos) = queue.pop_front() {
if pos == target {
return true;
}

for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
let next = (pos.0 + dx, pos.1 + dy);

if next.0 >= 0 && next.0 <= 70 &&
next.1 >= 0 && next.1 <= 70 &&
!corrupted.contains(&next) &&
!visited.contains(&next) {
visited.insert(next);
queue.push_back(next);
}
}
}

false
}