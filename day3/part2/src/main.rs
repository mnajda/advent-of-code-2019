extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};

struct Action {
    direction: char,
    value: i32,
}

#[derive(Clone, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
    len: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Hash for Point {
    fn hash<S: Hasher>(&self, state: &mut S) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn create_actions(input: &str) -> Vec<Action> {
    return input
        .split(',')
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let dir: char = chars[0];
            let val: String = chars[1..].iter().collect();

            Action {
                direction: dir,
                value: val.parse().unwrap(),
            }
        })
        .collect();
}

fn tokenize(filepath: &String) -> (Vec<Action>, Vec<Action>) {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    let (first, second) = contents.split_whitespace().next_tuple().unwrap();

    return (create_actions(&first), create_actions(&second));
}

fn apply(point: &mut Point, direction: &char) {
    match direction {
        'R' => point.x += 1,
        'L' => point.x -= 1,
        'U' => point.y += 1,
        'D' => point.y -= 1,
        _ => panic!("Unexpected operation"),
    };
    point.len += 1;
}

fn create_all_points(moves: Vec<Action>) -> HashSet<Point> {
    let mut points: Vec<Point> = vec![Point { x: 0, y: 0, len: 0 }];

    for action in moves {
        for _ in 0..action.value {
            let mut point: Point = points.last().unwrap().clone();
            apply(&mut point, &action.direction);
            points.push(point);
        }
    }
    points.remove(0usize);
    return points.drain(..).collect();
}

fn calculate_steps(lhs: &Point, rhs: &Point) -> i32 {
    return lhs.len + rhs.len;
}

fn solve(input: (Vec<Action>, Vec<Action>)) -> i32 {
    let first_wire: HashSet<Point> = create_all_points(input.0);
    let second_wire: HashSet<Point> = create_all_points(input.1);

    return first_wire
        .intersection(&second_wire)
        .map(|point| {
            calculate_steps(
                &first_wire.get(&point).unwrap(),
                &second_wire.get(&point).unwrap(),
            )
        })
        .min()
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let input = tokenize(filepath);
    let result = solve(input);
    println!("{}", result);
}
