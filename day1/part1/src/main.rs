use std::env;
use std::fs;

fn tokenize(filepath: &String) -> Vec<i32> {
    let contents = fs::read_to_string(filepath)
        .expect("Error reading file");
    return contents
                .split_whitespace()
                .map(|input| input.parse().unwrap())
                .collect();
}

fn calculate_result(input: &Vec<i32>) -> i32 {
    let denominator = 3;
    let constant = 2;

    return input.iter().fold(0i32, |sum, val| { sum + (val / denominator) - constant });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    
    let input = tokenize(filepath);
    let result = calculate_result(&input);
    println!("{}", result);
}
