use std::env;
use std::fs;

mod intcode;

fn tokenize(filepath: &String) -> Vec<i64> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn get_full_output(machine: &mut intcode::Machine) -> Vec<(i64, (i64, i64))> {
    let mut outputs: Vec<i64> = Vec::new();
    loop {
        match machine.pop_output() {
            Some(item) => outputs.push(item),
            None => break,
        }
    }

    return outputs.as_slice().chunks(3).map(|c| (c[2], (c[0], c[1]))).collect();
}

fn solve(mut program: Vec<i64>) {
    program[0] = 2;
    let mut machine = intcode::create_new_machine(program);
    let mut score = 0;
    let mut ball: Option<(i64, i64)> = None;
    let mut paddle: Option<(i64, i64)> = None;
    let mut input: i64 = 0;

    while !machine.is_finished() {
        machine.execute_program();

        let output = get_full_output(&mut machine);

        for tile in output {
            if (tile.1).0 == -1 && (tile.1).1 == 0 {
                score = tile.0;
            } else if tile.0 == 3 {
                paddle = Some(((tile.1).0, (tile.1).1));
            } else if tile.0 == 4 {
                ball = Some(((tile.1).0, (tile.1).1));
            }
    
            if ball != None && paddle != None {
                if ball.unwrap().0 < paddle.unwrap().0 {
                    input = -1;
                } else if ball.unwrap().0 > paddle.unwrap().0 {
                    input = 1;
                } else {
                    input = 0;
                }
            }
        }

        machine.push_input(input);
    }

    println!("{}", score);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];

    let program = tokenize(filepath);
    solve(program);
}
