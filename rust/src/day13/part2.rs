pub fn solve(input: &str) -> i64 {
    let mut total_tokens = 0;

    for machine in input.trim().split("\n\n") {
        let lines: Vec<_> = machine.lines().collect();

        let (ax, ay) = parse_button(lines[0]);
        let (bx, by) = parse_button(lines[1]);
        let (px, py) = parse_prize(lines[2]);

        if let Some(tokens) =
            solve_machine(ax, ay, bx, by, px + 10000000000000, py + 10000000000000)
        {
            total_tokens += tokens;
        }
    }

    total_tokens
}

fn parse_button(line: &str) -> (i64, i64) {
    let parts: Vec<_> = line.split(", ").collect();
    let x = parts[0].split("+").nth(1).unwrap().parse().unwrap();
    let y = parts[1].split("+").nth(1).unwrap().parse().unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (i64, i64) {
    let parts: Vec<_> = line.split(", ").collect();
    let x = parts[0].split("=").nth(1).unwrap().parse().unwrap();
    let y = parts[1].split("=").nth(1).unwrap().parse().unwrap();
    (x, y)
}

fn solve_machine(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> Option<i64> {
    let det = ax * by - ay * bx;
    if det == 0 {
        return None;
    }

    let a = ((px * by - py * bx) as f64 / det as f64).round() as i64;
    let b = ((py * ax - px * ay) as f64 / det as f64).round() as i64;

    if a * ax + b * bx != px || a * ay + b * by != py {
        return None;
    }

    if a < 0 || b < 0 {
        return None;
    }

    Some(3 * a + b)
}
