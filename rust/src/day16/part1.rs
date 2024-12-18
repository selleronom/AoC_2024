use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    row: usize,
    col: usize,
    dir: Direction,
    score: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

pub fn solve(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'S' {
                start = (i, j);
            }
            if grid[i][j] == 'E' {
                end = (i, j);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    heap.push(Reverse(State {
        row: start.0,
        col: start.1,
        dir: Direction::East,
        score: 0,
    }));

    while let Some(Reverse(state)) = heap.pop() {
        if (state.row, state.col) == end {
            return state.score;
        }

        if seen.contains(&(state.row, state.col, state.dir)) {
            continue;
        }
        seen.insert((state.row, state.col, state.dir));

        let (next_row, next_col) = match state.dir {
            Direction::North => (state.row.wrapping_sub(1), state.col),
            Direction::South => (state.row + 1, state.col),
            Direction::East => (state.row, state.col + 1),
            Direction::West => (state.row, state.col.wrapping_sub(1)),
        };

        if next_row < rows && next_col < cols && grid[next_row][next_col] != '#' {
            heap.push(Reverse(State {
                row: next_row,
                col: next_col,
                dir: state.dir,
                score: state.score + 1,
            }));
        }

        let left_dir = match state.dir {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        };
        heap.push(Reverse(State {
            row: state.row,
            col: state.col,
            dir: left_dir,
            score: state.score + 1000,
        }));

        let right_dir = match state.dir {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        };
        heap.push(Reverse(State {
            row: state.row,
            col: state.col,
            dir: right_dir,
            score: state.score + 1000,
        }));
    }

    0
}
