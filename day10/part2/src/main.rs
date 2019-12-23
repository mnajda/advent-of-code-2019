use std::env;
use std::fs;

#[derive(Debug, Clone, Copy)]
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
                    y: i as i32,
                    x: j as i32,
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

fn is_visible(a: &Point, b: &Point, c: &Point) -> bool {
    if is_between(a, b, c) {
        if get_length(a, c) < get_length(a, b) {
            if a.x - b.x < 0 && a.x - c.x < 0 {
                return false;
            } else if a.x - b.x > 0 && a.x - c.x > 0 {
                return false;
            } else if a.y - b.y < 0 && a.y - c.y < 0 {
                return false;
            } else if a.y - b.y > 0 && a.y - c.y > 0 {
                return false;
            }
        }
    }
    return true;
}

fn get_station_position(points: &Vec<Point>) -> Point {
    let mut best: (usize, Point) = (0, points[0]);

    for i in 0..points.len() {
        let mut visibile_from_point = (points.len() - 1, points[i]);
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

                if !is_visible(a, b, c) {
                    visibile_from_point.0 -= 1;
                    break;
                }
            }
        }

        if visibile_from_point.0 > best.0 {
            best = visibile_from_point;
        }
    }

    return best.1;
}

fn get_angles(station: &Point, points: &Vec<Point>) -> Vec<(f64, Point)> {
    let mut angles: Vec<(f64, Point)> = points
        .iter()
        .map(|item| {
            let angle =
                (station.y as f64 - item.y as f64).atan2(-(station.x as f64 - item.x as f64));

            return (angle, item.clone());
        })
        .collect();
    angles
        .as_mut_slice()
        .sort_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).unwrap());

    return angles;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("cargo run <filename>");
        return;
    }
    let filepath = &args[1];
    let map = load_map(filepath);
    let points = convert_to_points(map);
    let station = get_station_position(&points);

    let angles = get_angles(&station, &points);

    println!("{:?}", angles[200]);
    println!("{}", (angles[200].1.x * 100) + angles[200].1.y);
}
