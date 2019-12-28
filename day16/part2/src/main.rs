use std::env;
use std::fs;

fn tokenize(filepath: &String) -> (usize, Vec<i32>) {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    let offset: usize = contents[0..7].parse().unwrap();
    let mut output = contents.clone();
    for _ in 0..9999 {
        output.push_str(&contents);
    }
    return (
        offset,
        output
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect(),
    );
}

fn phase(input: Vec<i32>, offset: usize) -> Vec<i32> {
    let signal_length = input.len();
    let mut output: Vec<i32> = input;

    for i in (offset..signal_length).rev() {
        output[i - 1] = (output[i] + output[i - 1]) % 10;
    }

    return output;
}

fn fft(input: Vec<i32>, offset: usize) {
    let mut signal = input;

    for _ in 0..100 {
        signal = phase(signal, offset);
    }

    let result = signal[offset..offset + 8]
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
    let (offset, input) = tokenize(filepath);

    fft(input, offset);
}
