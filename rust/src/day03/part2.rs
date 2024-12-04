use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let pattern = r"do\(\)|don't\(\)|mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)";
    let re = Regex::new(pattern).unwrap();

    let mut enabled = true;
    let mut total = 0;

    for cap in re.captures_iter(input) {
        let instr = cap.get(0).unwrap().as_str();

        if instr == "do()" {
            enabled = true;
        } else if instr == "don't()" {
            enabled = false;
        } else {
            // This is a valid mul(X,Y) instruction
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();

            if enabled {
                total += x * y;
            }
        }
    }
    total // Return the total
}
