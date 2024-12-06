use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(&self, pos: &Position) -> Position {
        match self {
            Direction::Up => Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::Right => Position {
                x: pos.x + 1,
                y: pos.y,
            },
            Direction::Down => Position {
                x: pos.x,
                y: pos.y + 1,
            },
            Direction::Left => Position {
                x: pos.x - 1,
                y: pos.y,
            },
        }
    }
}

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    // Find starting position and direction
    let mut start_pos = Position { x: 0, y: 0 };
    let mut start_dir = Direction::Up;
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                start_pos = Position {
                    x: x as i32,
                    y: y as i32,
                };
                start_dir = Direction::Up;
            }
        }
    }

    let mut possible_positions = HashSet::new();

    // Try placing an obstruction at each empty position
    for y in 0..height {
        for x in 0..width {
            if grid[y as usize][x as usize] == '.' {
                let pos = Position { x, y };
                // Don't place obstruction at start position
                if pos != start_pos {
                    // Create temporary grid with new obstruction
                    let mut test_grid = grid.clone();
                    test_grid[y as usize][x as usize] = '#';

                    // Check if this creates a loop
                    if creates_loop(&test_grid, start_pos, start_dir, width, height) {
                        possible_positions.insert(pos);
                    }
                }
            }
        }
    }

    possible_positions.len()
}

fn creates_loop(
    grid: &Vec<Vec<char>>,
    start_pos: Position,
    start_dir: Direction,
    width: i32,
    height: i32,
) -> bool {
    let mut visited = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;

    visited.insert((pos, dir));

    loop {
        let next_pos = dir.move_forward(&pos);

        // Check if out of bounds
        if next_pos.x < 0 || next_pos.x >= width || next_pos.y < 0 || next_pos.y >= height {
            return false;
        }

        // Check if hit obstacle
        if grid[next_pos.y as usize][next_pos.x as usize] == '#' {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
        }

        // Check if we've seen this state before (indicating a loop)
        if !visited.insert((pos, dir)) {
            return true;
        }

        // Limit loop detection to prevent infinite loops
        if visited.len() > (width * height) as usize * 4 {
            return false;
        }
    }
}
