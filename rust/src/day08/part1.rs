use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut antennas: Vec<(usize, usize, char)> = Vec::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch.is_alphabetic() || ch.is_digit(10) {
                antennas.push((x, y, ch))
            }
        }
    }

    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i != j {
                let (x1, y1, f1) = antennas[i];
                let (x2, y2, f2) = antennas[j];

                if f1 == f2 {
                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;

                    let mx = x1 as isize - dx;
                    let my = y1 as isize - dy;
                    let px = x2 as isize + dx;
                    let py = y2 as isize + dy;

                    if mx >= 0
                        && my >= 0
                        && (mx as usize) < lines[0].len()
                        && (my as usize) < lines.len()
                    {
                        antinodes.insert((mx as usize, my as usize));
                    }

                    if px >= 0
                        && py >= 0
                        && (px as usize) < lines[0].len()
                        && (py as usize) < lines.len()
                    {
                        antinodes.insert((px as usize, py as usize));
                    }
                }
            }
        }
    }

    antinodes.len()
}
