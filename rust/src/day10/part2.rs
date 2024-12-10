use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> i32 {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    let mut sum_of_ratings = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                sum_of_ratings += calculate_trailhead_rating(&map, i, j);
            }
        }
    }

    sum_of_ratings
}

fn calculate_trailhead_rating(map: &Vec<Vec<u8>>, start_i: usize, start_j: usize) -> i32 {
    let mut distinct_trails = HashSet::new();
    let mut queue = VecDeque::new();
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    queue.push_back((start_i, start_j, vec![(start_i, start_j)]));

    while let Some((i, j, trail)) = queue.pop_front() {
        let current_height = map[i][j];

        if current_height == 9 {
            distinct_trails.insert(trail.clone());
            continue;
        }

        for &(di, dj) in directions.iter() {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni >= 0 && ni < map.len() as isize && nj >= 0 && nj < map[0].len() as isize {
                let ni = ni as usize;
                let nj = nj as usize;
                let next_height = map[ni][nj];

                if next_height == current_height + 1 && !trail.contains(&(ni, nj)) {
                    let mut new_trail = trail.clone();
                    new_trail.push((ni, nj));
                    queue.push_back((ni, nj, new_trail));
                }
            }
        }
    }

    distinct_trails.len() as i32
}
