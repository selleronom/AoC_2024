use std::collections::HashMap;

pub fn solve(input: &str) -> i32 {
    let mut total_tokens = 0;

    for machine in input.trim().split("\n\n") {
        let mut lines = machine.lines();

        let button_a = parse_button(lines.next().unwrap());
        let button_b = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());

        if let Some(tokens) = solve_machine(button_a, button_b, prize) {
            total_tokens += tokens;
        }
    }

    total_tokens
}

fn parse_button(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split(", ").collect();
    let x = parts[0].split("+").nth(1).unwrap().parse::<i32>().unwrap();
    let y = parts[1].split("+").nth(1).unwrap().parse::<i32>().unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split(", ").collect();
    let x = parts[0].split("=").nth(1).unwrap().parse::<i32>().unwrap();
    let y = parts[1].split("=").nth(1).unwrap().parse::<i32>().unwrap();
    (x, y)
}

fn solve_machine(button_a: (i32, i32), button_b: (i32, i32), prize: (i32, i32)) -> Option<i32> {
    let (ax, ay) = button_a;
    let (bx, by) = button_b;
    let (px, py) = prize;

    for a in 0..=100 {
        for b in 0..=100 {
            if a * ax + b * bx == px && a * ay + b * by == py {
                return Some(a * 3 + b);
            }
        }
    }
    None
}
