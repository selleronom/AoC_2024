use std::collections::HashMap;

pub fn solve(input: &str) -> i64 {
    let mut blocks: Vec<i64> = Vec::new();
    let mut file_lengths: HashMap<i64, i64> = HashMap::new();
    let mut file_id = 0;
    let mut is_file = true;

    // Parse input and create initial blocks
    for c in input.trim().chars() {
        let length = c.to_digit(10).unwrap() as i64;

        if is_file {
            blocks.extend(vec![file_id; length as usize]);
            file_lengths.insert(file_id, length);
            file_id += 1;
        } else {
            blocks.extend(vec![-1; length as usize]); // Use -1 to represent '.'
        }
        is_file = !is_file;
    }

    // Process files in decreasing order of file ID
    if let Some(&max_file_id) = file_lengths.keys().max() {
        for current_file_id in (0..=max_file_id).rev() {
            if !file_lengths.contains_key(&current_file_id) {
                continue;
            }

            let file_length = *file_lengths.get(&current_file_id).unwrap();

            // Find file start position
            let file_start = match blocks.iter().position(|&x| x == current_file_id) {
                Some(pos) => pos,
                None => continue,
            };

            // Look for leftmost suitable free space
            let mut free_space_start = None;
            let mut consecutive_free = 0;
            let mut found_suitable_space = false;

            for (i, &block) in blocks.iter().take(file_start).enumerate() {
                if block == -1 {
                    if consecutive_free == 0 {
                        free_space_start = Some(i);
                    }
                    consecutive_free += 1;
                    if consecutive_free >= file_length {
                        found_suitable_space = true;
                        break;
                    }
                } else {
                    consecutive_free = 0;
                    free_space_start = None;
                }
            }

            // Move file if suitable space found
            if let Some(free_start) = free_space_start {
                if found_suitable_space && free_start < file_start {
                    // Clear original location
                    for i in file_start..file_start + file_length as usize {
                        blocks[i] = -1;
                    }
                    // Place in new location
                    for i in free_start..free_start + file_length as usize {
                        blocks[i] = current_file_id;
                    }
                }
            }
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
