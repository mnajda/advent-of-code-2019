use std::env;
use std::fs;

fn tokenize(filepath: &String) -> Vec<i32> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn execute_operation(op: i32, pos: usize, input: &mut Vec<i32>) {
    let lhs = input[input[pos + 1] as usize];
    let rhs = input[input[pos + 2] as usize];
    let dest = input[pos + 3] as usize;

    match op {
        1 => {
            input[dest] = lhs + rhs;
        }
        2 => {
            input[dest] = lhs * rhs;
        }
        _ => {
            panic!("Unexpected operation");
        }
    }
}

fn execute_program(input: &mut Vec<i32>) {
    for i in (16..input.len()).step_by(4) {
        if input[i] == 99 {
            break;
        }
        execute_operation(input[i], i, input);
    }
}

fn bruteforce(input: &Vec<i32>) {
    for i in 0..100 {
        for j in 0..100 {
            let mut vec = input.clone();
            vec[1] = i;
            vec[2] = j;
            execute_program(&mut vec);
            if vec[0] == 19690720 {
                println!("{}", (100 * vec[1]) + vec[2]);
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let mut input = tokenize(filepath);

    bruteforce(&mut input);
}
