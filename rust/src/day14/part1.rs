
#[derive(Debug, Clone)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

pub fn solve(input: &str) -> i64 {
    let mut robots = parse_input(input);
    let width = 101;
    let height = 103;

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.pos.0 = (robot.pos.0 + robot.vel.0).rem_euclid(width);
            robot.pos.1 = (robot.pos.1 + robot.vel.1).rem_euclid(height);
        }
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in robots {
        let x = robot.pos.0;
        let y = robot.pos.1;

        if x == width / 2 || y == height / 2 {
            continue;
        }

        match (x < width / 2, y < height / 2) {
            (true, true) => q1 += 1,
            (false, true) => q2 += 1,
            (true, false) => q3 += 1,
            (false, false) => q4 += 1,
        };
    }

    q1 * q2 * q3 * q4
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let pos = parts
                .next()
                .unwrap()
                .trim_start_matches("p=")
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();

            let vel = parts
                .next()
                .unwrap()
                .trim_start_matches("v=")
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();

            Robot {
                pos: (pos[0], pos[1]),
                vel: (vel[0], vel[1]),
            }
        })
        .collect()
}
