use std::fs;

mod day02 {
    pub mod part1;
    pub mod part2;
}

fn main() {
    let input = fs::read_to_string("../../input/day02.txt").expect("Failed to read input");
    println!("Part 1: {}", day02::part1::solve(&input));
    println!("Part 2: {}", day02::part2::solve(&input));
}
