pub fn solve(input_data: &str) -> usize {
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // down-right
        (1, -1),  // down-left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // up-left
        (-1, 1),  // up-right
    ];

    let grid: Vec<Vec<char>> = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    let word = "XMAS";
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for (dr, dc) in directions.iter() {
                if let Some(true) = (0..word_len)
                    .map(|i| {
                        let nr = row as isize + dr * i as isize;
                        let nc = col as isize + dc * i as isize;

                        if nr >= 0
                            && nr < grid.len() as isize
                            && nc >= 0
                            && nc < grid[0].len() as isize
                        {
                            grid[nr as usize][nc as usize] == word_chars[i]
                        } else {
                            false
                        }
                    })
                    .reduce(|acc, x| acc && x)
                {
                    count += 1;
                }
            }
        }
    }

    count
}
