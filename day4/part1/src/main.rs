use std::env;

fn convert_to_digits(password: &u32) -> Vec<u32> {
    return password
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
}

fn is_sorted(password: &Vec<u32>) -> bool {
    return password.windows(2).all(|window| window[0] <= window[1]);
}

fn has_two_eq_adjacent_digits(password: &Vec<u32>) -> bool {
    return password
        .windows(2)
        .map(|window| window[0] == window[1])
        .any(|item| item == true);
}

fn is_valid(password: &u32) -> bool {
    let digits = convert_to_digits(&password);
    return is_sorted(&digits) && has_two_eq_adjacent_digits(&digits);
}

fn solve(lower_bound: u32, upper_bound: u32) -> usize {
    let range = lower_bound..upper_bound;

    return range.filter(|password| is_valid(password)).count();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Provide range");
        return;
    }
    let lower_bound: u32 = args[1].parse().unwrap();
    let upper_bound: u32 = args[2].parse().unwrap();

    let result = solve(lower_bound, upper_bound);
    println!("{}", result);
}
