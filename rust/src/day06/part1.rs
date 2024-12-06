use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
        }
    }
}

pub fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut guard_pos = (0, 0);
    let mut guard_dir = Direction::Up;

    for (y, line) in lines.iter().enumerate() {
        if let Some(x) = line.find('^') {
            guard_pos = (x as i32, y as i32);
            break;
        }
    }

    let mut visited_positions = HashSet::new();
    visited_positions.insert(guard_pos);

    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    loop {
        let next_pos = guard_dir.move_forward(guard_pos);

        if next_pos.0 >= 0
            && next_pos.0 < width as i32
            && next_pos.1 >= 0
            && next_pos.1 < height as i32
        {
            if map[next_pos.1 as usize][next_pos.0 as usize] == '#' {
                guard_dir = guard_dir.turn_right();
            } else {
                guard_pos = next_pos;
                visited_positions.insert(guard_pos);
            }
        } else {
            break; // Guard goes out of bounds
        }
    }

    visited_positions.len()
}
