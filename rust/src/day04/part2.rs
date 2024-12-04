// part2.rs
pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if grid[i][j] == 'A' {
                let patterns = [['M', 'A', 'S'], ['S', 'A', 'M']];

                for p1 in &patterns {
                    for p2 in &patterns {
                        // Check top-left to bottom-right
                        if (grid[i-1][j-1] == p1[0] && grid[i+1][j+1] == p1[2]) &&
                           // Check top-right to bottom-left
                           (grid[i-1][j+1] == p2[0] && grid[i+1][j-1] == p2[2])
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}
