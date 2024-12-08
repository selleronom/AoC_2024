use num::integer::gcd;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let lines: Vec<&str> = input.lines().collect();
    let grid_height = lines.len() as i32;
    let grid_width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for positions in antennas.values() {
        let num_antennas = positions.len();
        if num_antennas < 2 {
            continue;
        }
        for i in 0..num_antennas {
            for j in i + 1..num_antennas {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let gcd_xy = gcd(dx, dy);

                let step_x = dx / gcd_xy;
                let step_y = dy / gcd_xy;

                let max_t = grid_width.max(grid_height) * 2;

                for t in -max_t..=max_t {
                    let x = x1 + t * step_x;
                    let y = y1 + t * step_y;

                    if x >= 0 && x < grid_width && y >= 0 && y < grid_height {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
    }

    antinodes.len()
}
