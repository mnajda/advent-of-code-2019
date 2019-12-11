use std::env;
use std::fs;

mod intcode_machine;

fn tokenize(filepath: &String) -> Vec<i64> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Provide filename and input");
        return;
    }
    let filepath = &args[1];
    let input: i64 = args[2].parse().unwrap();

    let program = tokenize(filepath);
    let mut machine = intcode_machine::create_new_machine(program);

    machine.push_input(input);
    machine.execute_program();

    println!("{}", machine.pop_output().unwrap());
}
