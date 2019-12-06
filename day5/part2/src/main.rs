use std::env;
use std::fs;

fn tokenize(filepath: &String) -> Vec<i32> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn get_operation_size(op: &i32) -> usize {
    match op {
        1 | 2 | 7 | 8 => 4,
        3 | 4 => 2,
        5 | 6 => 3,
        _ => panic!("Unexpected operation"),
    }
}

fn get_addressing(input: i32, op_size: usize) -> Vec<i32> {
    let mut temp = input;
    let mut addressing: Vec<i32> = Vec::new();

    for _ in 0..op_size - 1 {
        addressing.push(temp % 10);
        temp /= 10;
    }

    return addressing;
}

fn get_value(addressing: i32, pos: usize, program: &Vec<i32>) -> i32 {
    if addressing == 0 {
        return program[program[pos] as usize];
    } else {
        return program[pos];
    }
}

fn execute_operation(
    op: i32,
    ip: &mut usize,
    program: &mut Vec<i32>,
    input: &i32,
    addressing: Vec<i32>,
) {
    // TODO refactor this awful function
    match op {
        1 => {
            let lhs = get_value(addressing[0], *ip + 1, program);
            let rhs = get_value(addressing[1], *ip + 2, program);
            let dest = program[*ip + 3] as usize;

            program[dest] = lhs + rhs;
            *ip += get_operation_size(&op);
        }
        2 => {
            let lhs = get_value(addressing[0], *ip + 1, program);
            let rhs = get_value(addressing[1], *ip + 2, program);
            let dest = program[*ip + 3] as usize;

            program[dest] = lhs * rhs;
            *ip += get_operation_size(&op);
        }
        3 => {
            let dest = program[*ip + 1] as usize;
            program[dest] = *input;

            *ip += get_operation_size(&op);
        }
        4 => {
            let source = get_value(addressing[0], *ip + 1, program) as usize;
            println!("{}", source);

            *ip += get_operation_size(&op);
        }
        5 => {
            let condition = get_value(addressing[0], *ip + 1, program);
            let dest = get_value(addressing[1], *ip + 2, program);

            if condition != 0 {
                *ip = dest as usize;
            }
            else {
                *ip += get_operation_size(&op);
            }
        }
        6 => {
            let condition = get_value(addressing[0], *ip + 1, program);
            let dest = get_value(addressing[1], *ip + 2, program);

            if condition == 0 {
                *ip = dest as usize;
            }
            else {
                *ip += get_operation_size(&op);
            }
        }
        7 => {
            let lhs = get_value(addressing[0], *ip + 1, program);
            let rhs = get_value(addressing[1], *ip + 2, program);
            let dest = program[*ip + 3] as usize;

            if lhs < rhs {
                program[dest] = 1;
            }
            else {
                program[dest] = 0;
            }

            *ip += get_operation_size(&op);
        }
        8 => {
            let lhs = get_value(addressing[0], *ip + 1, program);
            let rhs = get_value(addressing[1], *ip + 2, program);
            let dest = program[*ip + 3] as usize;

            if lhs == rhs {
                program[dest] = 1;
            }
            else {
                program[dest] = 0;
            }

            *ip += get_operation_size(&op);
        }
        _ => {
            panic!("Unexpected operation");
        }
    }
}

fn execute_program(program: &mut Vec<i32>, input: i32) {
    let mut ip: usize = 0;
    while ip < program.len() {
        if program[ip] == 99 {
            break;
        }
        let op = program[ip] % 100;
        let op_size = get_operation_size(&op);
        let addressing_modes = program[ip] / 100;
        execute_operation(
            op,
            &mut ip,
            program,
            &input,
            get_addressing(addressing_modes, op_size),
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Provide filename and input");
        return;
    }
    let filepath = &args[1];
    let input: i32 = args[2].parse().unwrap();

    let mut program = tokenize(filepath);
    execute_program(&mut program, input);
}
