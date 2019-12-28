use std::env;
use std::fs;

fn tokenize(filepath: &String) -> Vec<i32> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
}

fn generate_pattern(pos: usize, length: usize) -> Vec<i32> {
    let base = vec![0, 1, 0, -1];
    let mut pattern: Vec<i32> = Vec::new();
    let mut offset = 0;

    for _ in 0..length + 1 {
        for _ in 0..pos + 1 {
            pattern.push(base[offset]);
        }
        offset = (offset + 1) % base.len();
    }

    return pattern[1..length + 1].to_vec();
}

fn generate_cache(length: usize) -> Vec<Vec<i32>> {
    let mut cache: Vec<Vec<i32>> = Vec::with_capacity(length);

    for i in 0..length {
        let pattern = generate_pattern(i, length);

        cache.push(pattern);
    }

    return cache;
}

fn phase(input: Vec<i32>, cache: &Vec<Vec<i32>>) -> Vec<i32> {
    let signal_length = input.len();
    let mut output: Vec<i32> = Vec::with_capacity(signal_length);

    for i in 0..signal_length {
        let pattern = &cache[i];

        let mut value = 0;
        for k in 0..signal_length {
            value += pattern[k] * input[k];
        }

        let last_digit = if value < 0 {
            (value * -1) % 10
        } else {
            value % 10
        };

        output.push(last_digit);
    }

    return output;
}

fn fft(input: Vec<i32>) {
    let cache = generate_cache(input.len());

    let mut signal = input;

    for _ in 0..100 {
        signal = phase(signal, &cache);
    }

    let result = signal[0..8]
        .iter()
        .fold("".to_owned(), |acc, item| acc + &item.to_string());

    println!("{}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let input = tokenize(filepath);

    fft(input);
}
