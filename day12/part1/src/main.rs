extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug, Default)]
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

fn simulate(input: Vec<Moon>) {
    let mut moons = input;
    for _ in 0..1000 {
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
    }

    let mut total = 0;

    for i in 0..moons.len() {
        println!("{:?}", moons[i]);
        
        let pot = i32::abs(moons[i].x) + i32::abs(moons[i].y) + i32::abs(moons[i].z);
        let kin = i32::abs(moons[i].velocity.0) + i32::abs(moons[i].velocity.1) + i32::abs(moons[i].velocity.2);

        total += pot * kin;
    }

    println!("{}", total);

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
