use itertools::Itertools;
use std::env;
use std::fs;

fn tokenize(filepath: &String, layer_size: usize) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .chars()
        .chunks(layer_size)
        .into_iter()
        .map(|chunk| chunk.collect::<String>().chars().collect())
        .collect();
}

fn count_zeros(input: &Vec<char>) -> usize {
    return input.iter().filter(|&item| *item == '0').count();
}

fn solve(input: &mut Vec<Vec<char>>) -> usize {
    input.sort_by(|lhs, rhs| count_zeros(lhs).partial_cmp(&count_zeros(rhs)).unwrap());

    let ones_count = input[0].iter().filter(|&item| *item == '1').count();
    let twos_count = input[0].iter().filter(|&item| *item == '2').count();

    return ones_count * twos_count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("run <filepath> <width> <height>");
        return;
    }
    let filepath = &args[1];
    let width: usize = args[2].parse().unwrap();
    let height: usize = args[3].parse().unwrap();
    let layer_size = width * height;

    let mut input = tokenize(filepath, layer_size);
    let result = solve(&mut input);
    println!("{}", result);
}
