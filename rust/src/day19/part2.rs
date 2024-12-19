use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
let mut lines = input.lines();

let patterns: HashSet<&str> = lines
.next()
.unwrap()
.split(", ")
.collect();

lines.next();

lines
.map(|design| count_arrangements(design, &patterns, &mut HashMap::new()))
.sum()
}

fn count_arrangements<'a>(
design: &'a str,
patterns: &HashSet<&str>,
cache: &mut HashMap<&'a str, usize>
) -> usize {
if design.is_empty() {
return 1;
}

if let Some(&count) = cache.get(design) {
return count;
}

let mut total = 0;
for pattern in patterns {
if design.starts_with(pattern) {
total += count_arrangements(&design[pattern.len()..], patterns, cache);
}
}

cache.insert(design, total);
total
}