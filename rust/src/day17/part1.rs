
pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let program_line = lines.find(|line| line.starts_with("Program:")).unwrap();
    let program: Vec<u8> = program_line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect();

    let mut registers = Registers { a, b, c };
    let mut ip: usize = 0;
    let mut output = Vec::new();

    while ip < program.len() {
        let opcode = program[ip];
        if ip + 1 >= program.len() {
            break;
        }
        let operand = program[ip + 1];
        match opcode {
            0 => {
                // adv
                let denom = 2i64.pow(get_combo_operand(operand, &registers));
                registers.a = registers.a / denom;
                ip += 2;
            }
            1 => {
                // bxl
                let literal = operand as i64;
                registers.b ^= literal;
                ip += 2;
            }
            2 => {
                // bst
                let val = get_combo_operand(operand, &registers) % 8;
                registers.b = val as i64;
                ip += 2;
            }
            3 => {
                // jnz
                if registers.a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                // bxc
                registers.b ^= registers.c;
                ip += 2;
            }
            5 => {
                // out
                let val = get_combo_operand(operand, &registers) % 8;
                output.push(val.to_string());
                ip += 2;
            }
            6 => {
                // bdv
                let denom = 2i64.pow(get_combo_operand(operand, &registers));
                registers.b = registers.a / denom;
                ip += 2;
            }
            7 => {
                // cdv
                let denom = 2i64.pow(get_combo_operand(operand, &registers));
                registers.c = registers.a / denom;
                ip += 2;
            }
            _ => {
                // Invalid opcode
                break;
            }
        }
    }

    output.join(",")
}

struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

fn get_combo_operand(operand: u8, registers: &Registers) -> u32 {
    match operand {
        0..=3 => operand as u32,
        4 => registers.a as u32,
        5 => registers.b as u32,
        6 => registers.c as u32,
        _ => 0, // Operand 7 is reserved and will not appear
    }
}
