pub fn solve(input: &str) -> i64 {
    let mut blocks: Vec<i64> = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    // Parse input and create initial blocks
    for c in input.trim().chars() {
        let length = c.to_digit(10).unwrap() as i64;

        if is_file {
            blocks.extend(vec![file_id; length as usize]);
            file_id += 1;
        } else {
            blocks.extend(vec![-1; length as usize]); // Use -1 to represent '.'
        }
        is_file = !is_file;
    }

    // Simulate the moving process
    loop {
        // Find leftmost free space
        let lfs = blocks.iter().position(|&x| x == -1);

        // Find rightmost file block
        let rfb = blocks
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &x)| x != -1)
            .map(|(i, _)| i);

        match (lfs, rfb) {
            (Some(lfs_pos), Some(rfb_pos)) if rfb_pos > lfs_pos => {
                // Move one block from rfb to lfs
                blocks[lfs_pos] = blocks[rfb_pos];
                blocks[rfb_pos] = -1;
            }
            _ => break, // No more moves possible
        }
    }

    // Calculate checksum
    blocks
        .iter()
        .enumerate()
        .filter(|&(_, &block)| block != -1)
        .map(|(pos, &block)| pos as i64 * block)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "3132"; // Example input
        assert_eq!(solve(input), 14); // Adjust expected value based on your test case
    }
}
