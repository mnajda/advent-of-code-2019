use std::collections::HashSet;
use std::env;
use std::fs;

fn tokenize(filepath: &String) -> Vec<(String, String)> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    let by_line: Vec<String> = contents
        .split_whitespace()
        .map(|line| line.to_string())
        .collect();
    return by_line
        .iter()
        .map(|item| {
            let temp: Vec<_> = item.split(")").collect();
            (temp[0].to_string(), temp[1].to_string())
        })
        .collect();
}

fn create_path(start_node: String, input: &Vec<(String, String)>) -> HashSet<String> {
    let mut path: Vec<String> = Vec::new();
    let mut node = start_node;

    loop {
        match input.iter().find(|item| item.1 == node) {
            Some(item) => {
                node = item.0.clone();
                path.push(item.0.clone());
            }
            None => break,
        }
    }

    return path.drain(..).collect();
}

fn solve(input: Vec<(String, String)>) -> usize {
    let santa_path = create_path("SAN".to_string(), &input);
    let my_path = create_path("YOU".to_string(), &input);

    return santa_path.symmetric_difference(&my_path).count();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let tokens = tokenize(filepath);

    let result = solve(tokens);
    println!("{}", result);
}
