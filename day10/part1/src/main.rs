use std::env;
use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn load_map(filepath: &String) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    let by_lines: Vec<String> = contents
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();

    return by_lines.iter().map(|line| line.chars().collect()).collect();
}

fn convert_to_points(map: Vec<Vec<char>>) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '#' {
                points.push(Point {
                    x: i as i32,
                    y: j as i32,
                });
            }
        }
    }

    return points;
}

fn is_between(a: &Point, b: &Point, c: &Point) -> bool {
    return ((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))) == 0;
}

fn get_length(a: &Point, b: &Point) -> i32 {
    return i32::abs(a.x - b.x) + i32::abs(a.y - b.y);
}

fn solve(points: &mut Vec<Point>) -> usize {
    let mut max = 0;

    for i in 0..points.len() {
        let mut visible = points.len() - 1;

        for j in 0..points.len() {
            if i == j {
                continue;
            }

            for k in 0..points.len() {
                if i == k || j == k {
                    continue;
                }

                let a = &points[i];
                let b = &points[j];
                let c = &points[k];

                if is_between(a, b, c) {
                    if get_length(a, c) < get_length(a, b) {
                        if a.x - b.x < 0 && a.x - c.x < 0 {
                            visible -= 1;
                            break;
                        }
                        else if a.x - b.x > 0 && a.x - c.x > 0 {
                            visible -= 1;
                            break;
                        }
                        else if a.y - b.y < 0 && a.y - c.y < 0 {
                            visible -= 1;
                            break;
                        }
                        else if a.y - b.y > 0 && a.y - c.y > 0 {
                            visible -= 1;
                            break;
                        }
                    }
                }
            }
        }

        if visible > max {
            max = visible;
        }
    }

    return max;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("cargo run <filename>");
        return;
    }
    let filepath = &args[1];
    let map = load_map(filepath);
    let mut points = convert_to_points(map);
    let result = solve(&mut points);

    println!("{}", result);
}
