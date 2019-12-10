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

fn get_pixel(index: usize, input: &Vec<Vec<char>>) -> char {
    let mut depth: usize = 0;
    while depth < input.len() {
        if input[depth][index] != '2' {
            return input[depth][index];
        }
        depth += 1;
    }
    panic!("Found only transparent pixels");
}

fn solve(input: Vec<Vec<char>>, layer_size: usize) -> Vec<char> {
    let mut output: Vec<char> = vec!['0'; layer_size];

    for i in 0..layer_size {
        let pixel = get_pixel(i, &input);
        output[i] = pixel;
    }

    return output;
}

fn into_image(input: &Vec<char>, width: usize) -> String {
    let mut image: String = String::new();

    for i in (0..input.len()).step_by(width) {
        let temp: String = input[i..i + width]
            .iter()
            .map(|item| match item {
                '0' => ' ',
                '1' => 'X',
                _ => panic!("Unexpected char"),
            })
            .collect();
        image.push_str(&temp);
        image.push_str("\n");
    }

    return image;
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

    let input = tokenize(filepath, layer_size);
    let result = solve(input, layer_size);

    let image = into_image(&result, width);
    println!("{}", image);
}
