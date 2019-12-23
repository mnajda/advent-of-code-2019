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

fn solve(program: Vec<i64>) {
    let block_tile_id = 2;
    let mut machine = intcode::create_new_machine(program);

    machine.execute_program();

    let tiles = get_full_output(&mut machine);

    let solution = tiles.iter().filter(|item| item.0 == block_tile_id).count();
    println!("{}", solution);
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
