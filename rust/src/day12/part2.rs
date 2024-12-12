use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = HashSet::new();
    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visited.contains(&(r, c)) {
                continue;
            }

            let current = grid[r][c];
            let mut region = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((r, c));
            visited.insert((r, c));

            while let Some((row, col)) = queue.pop_front() {
                region.insert((row as i32, col as i32));

                for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_r = row as i32 + dr;
                    let new_c = col as i32 + dc;

                    if new_r >= 0 && new_r < rows as i32 && new_c >= 0 && new_c < cols as i32 {
                        let nr = new_r as usize;
                        let nc = new_c as usize;

                        if grid[nr][nc] == current && !visited.contains(&(nr, nc)) {
                            queue.push_back((nr, nc));
                            visited.insert((nr, nc));
                        }
                    }
                }
            }

            if region.is_empty() {
                continue;
            }

            let mut corners = 0;

            // Check both types of corners
            for &(row, col) in &region {
                // External corners
                for [(dr1, dc1), (dr2, dc2)] in [
                    [(0, 1), (1, 0)],
                    [(1, 0), (0, -1)],
                    [(0, -1), (-1, 0)],
                    [(-1, 0), (0, 1)],
                ] {
                    let p1 = (row + dr1, col + dc1);
                    let p2 = (row + dr2, col + dc2);
                    if !region.contains(&p1) && !region.contains(&p2) {
                        corners += 1;
                    }
                }

                // Internal corners
                for [(dr1, dc1), (dr2, dc2)] in [
                    [(0, 1), (1, 0)],
                    [(1, 0), (0, -1)],
                    [(0, -1), (-1, 0)],
                    [(-1, 0), (0, 1)],
                ] {
                    let p1 = (row + dr1, col + dc1);
                    let p2 = (row + dr2, col + dc2);
                    let p3 = (row + dr1 + dr2, col + dc1 + dc2);
                    if region.contains(&p1) && region.contains(&p2) && !region.contains(&p3) {
                        corners += 1;
                    }
                }
            }

            total_price += region.len() as i32 * corners;
        }
    }

    total_price
}
