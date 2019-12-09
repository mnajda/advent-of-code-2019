use std::collections::HashMap;
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

fn count_orbits(object: &String, orbits: &HashMap<String, Vec<String>>) -> usize {
    if !orbits.contains_key(object) {
        return 0;
    }
    let next = orbits.get(object).unwrap();
    return next.len()
        + next
            .iter()
            .fold(0, |acc, item| acc + count_orbits(item, orbits));
}

fn solve(input: Vec<(String, String)>) -> usize {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();

    for object in input {
        if orbits.contains_key(&object.0) {
            orbits.get_mut(&object.0).unwrap().push(object.1);
            continue;
        }
        orbits.insert(object.0, vec![object.1]);
    }

    return orbits
        .iter()
        .fold(0, |acc, item| acc + count_orbits(&item.0, &orbits));
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
