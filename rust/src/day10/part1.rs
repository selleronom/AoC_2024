use std::collections::{HashSet, VecDeque};

pub fn solve(input_data: &str) -> usize {
    let map: Vec<Vec<u8>> = input_data
        .lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect();

    let mut total_score = 0;

    let rows = map.len();
    let cols = map[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                let mut score = 0;
                let mut queue = VecDeque::new();
                let mut visited = HashSet::new();
                queue.push_back((r, c, 0));
                visited.insert((r, c));

                while let Some((cur_r, cur_c, height)) = queue.pop_front() {
                    if height == 9 {
                        score += 1;
                        continue;
                    }

                    let potential_moves = vec![
                        (cur_r.wrapping_sub(1), cur_c),
                        (cur_r + 1, cur_c),
                        (cur_r, cur_c.wrapping_sub(1)),
                        (cur_r, cur_c + 1),
                    ];

                    for (next_r, next_c) in potential_moves {
                        if next_r < rows
                            && next_c < cols
                            && !visited.contains(&(next_r, next_c))
                            && map[next_r][next_c] == height + 1
                        {
                            visited.insert((next_r, next_c));
                            queue.push_back((next_r, next_c, height + 1));
                        }
                    }
                }

                total_score += score;
            }
        }
    }

    total_score
}
