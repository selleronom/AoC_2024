use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut total_price = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let plant_type = grid[i][j];
            let mut region_area = 0;
            let mut region_perimeter = 0;
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            visited.insert((i, j));

            while let Some((x, y)) = queue.pop_front() {
                region_area += 1;
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx < 0
                        || ny < 0
                        || nx >= grid.len() as isize
                        || ny >= grid[0].len() as isize
                        || grid[nx as usize][ny as usize] != plant_type
                    {
                        region_perimeter += 1;
                    } else if !visited.contains(&(nx as usize, ny as usize)) {
                        visited.insert((nx as usize, ny as usize));
                        queue.push_back((nx as usize, ny as usize));
                    }
                }
            }
            total_price += region_area * region_perimeter;
        }
    }

    total_price
}
