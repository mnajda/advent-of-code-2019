extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::env;
use std::fs;
use num_integer::lcm;

#[derive(Debug, Default, Clone, PartialEq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    velocity: (i32, i32, i32),
}

fn load_value(moon: &mut Moon, input: &str) {
    let operands: Vec<String> = input.split('=').map(|item| item.to_string()).collect();

    match operands[0].as_str() {
        "x" => moon.x = operands[1].parse().unwrap(),
        "y" => moon.y = operands[1].parse().unwrap(),
        "z" => moon.z = operands[1].parse().unwrap(),
        _ => panic!("Unexpected operand"),
    }
}

fn tokenize(filepath: &String) -> Vec<Moon> {
    let mut moons: Vec<Moon> = Vec::new();
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    let re = Regex::new(".[=]-?[0-9]+").unwrap();

    for chunk in &re.find_iter(&contents).chunks(3) {
        let mut moon = Moon::default();
        for part in chunk {
            load_value(&mut moon, part.as_str());
        }
        moons.push(moon);
    }

    return moons;
}

fn update_axis(lhs: i32, rhs: i32) -> i32 {
    if rhs > lhs {
        return 1;
    } else if rhs < lhs {
        return -1;
    } else {
        return 0;
    }
}

fn get_updated_velocity(lhs: &Moon, rhs: &Moon) -> (i32, i32, i32) {
    let vel_x = update_axis(lhs.x, rhs.x);
    let vel_y = update_axis(lhs.y, rhs.y);
    let vel_z = update_axis(lhs.z, rhs.z);

    return (vel_x, vel_y, vel_z);
}

fn update_position(moon: &mut Moon) {
    moon.x += moon.velocity.0;
    moon.y += moon.velocity.1;
    moon.z += moon.velocity.2;
}

fn compare_x(moons: &Vec<Moon>, previous: &Vec<Moon>) -> bool {
    return moons[0].x == previous[0].x
        && moons[1].x == previous[1].x
        && moons[2].x == previous[2].x
        && moons[3].x == previous[3].x
        && moons[0].velocity.0 == previous[0].velocity.0
        && moons[1].velocity.0 == previous[1].velocity.0
        && moons[2].velocity.0 == previous[2].velocity.0
        && moons[3].velocity.0 == previous[3].velocity.0;
}

fn compare_y(moons: &Vec<Moon>, previous: &Vec<Moon>) -> bool {
    return moons[0].y == previous[0].y
        && moons[1].y == previous[1].y
        && moons[2].y == previous[2].y
        && moons[3].y == previous[3].y
        && moons[0].velocity.1 == previous[0].velocity.1
        && moons[1].velocity.1 == previous[1].velocity.1
        && moons[2].velocity.1 == previous[2].velocity.1
        && moons[3].velocity.1 == previous[3].velocity.1;
}

fn compare_z(moons: &Vec<Moon>, previous: &Vec<Moon>) -> bool {
    return moons[0].z == previous[0].z
        && moons[1].z == previous[1].z
        && moons[2].z == previous[2].z
        && moons[3].z == previous[3].z
        && moons[0].velocity.2 == previous[0].velocity.2
        && moons[1].velocity.2 == previous[1].velocity.2
        && moons[2].velocity.2 == previous[2].velocity.2
        && moons[3].velocity.2 == previous[3].velocity.2;
}

fn simulate_axis(input: Vec<Moon>, comparator: &dyn Fn(&Vec<Moon>, &Vec<Moon>) -> bool) -> i64 {
    let mut moons = input.clone();
    let previous = moons.clone();
    let mut iteration: i64 = 0;

    loop {
        if iteration != 0 && comparator(&moons, &previous) {
            break;
        }
        for i in 0..moons.len() {
            for k in 0..moons.len() {
                if i == k {
                    continue;
                }
                let current_velocity = &moons[i].velocity;
                let new_velocity = get_updated_velocity(&moons[i], &moons[k]);

                moons[i].velocity = (
                    current_velocity.0 + new_velocity.0,
                    current_velocity.1 + new_velocity.1,
                    current_velocity.2 + new_velocity.2,
                );
            }
        }
        for i in 0..moons.len() {
            update_position(&mut moons[i]);
        }

        iteration += 1;
    }

    return iteration;
}

fn simulate(input: Vec<Moon>) {
    let iteration_x = simulate_axis(input.clone(), &compare_x);
    let iteration_y = simulate_axis(input.clone(), &compare_y);
    let iteration_z = simulate_axis(input.clone(), &compare_z);

    let solution = lcm(iteration_x, lcm(iteration_y, iteration_z));
    println!("{}", solution);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let moons = tokenize(filepath);
    simulate(moons);
}
