use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

mod intcode;

type Point = (i64, i64);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

fn tokenize(filepath: &String) -> Vec<i64> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn unwrap_output(output: Option<i64>) -> i64 {
    match output {
        Some(item) => item,
        None => panic!("Expected output value"),
    }
}

fn get_tile_type(id: i64) -> Tile {
    match id {
        0 => Tile::Wall,
        1 => Tile::Empty,
        2 => Tile::Oxygen,
        _ => panic!("Unexpected tile id"),
    }
}

fn get_new_position(direction: i64, old_position: &Point) -> Point {
    match direction {
        1 => (old_position.0, old_position.1 + 1),
        2 => (old_position.0, old_position.1 - 1),
        3 => (old_position.0 + 1, old_position.1),
        4 => (old_position.0 - 1, old_position.1),
        _ => panic!("Unexpected direction"),
    }
}

fn get_reverse_dir(direction: i64) -> i64 {
    match direction {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("Unexpected direction"),
    }
}

fn visit(machine: &mut intcode::Machine, map: &mut HashMap<Point, Tile>, pos: Point) {
    for dir in 1..5 {
        let new_pos = get_new_position(dir, &pos);

        if !map.contains_key(&new_pos) {
            machine.push_input(dir);
            machine.execute_program();

            let tile = get_tile_type(unwrap_output(machine.pop_output()));
            map.insert(new_pos, tile);

            if tile != Tile::Wall {
                visit(machine, map, new_pos);

                let reversed = get_reverse_dir(dir);

                machine.push_input(reversed);
                machine.execute_program();

                let reversed_tile = get_tile_type(unwrap_output(machine.pop_output()));
                let reversed_pos = get_new_position(dir, &pos);
                if !map.contains_key(&reversed_pos) {
                    map.insert(reversed_pos, reversed_tile);
                }
            }
        }
    }
}

fn generate_map(program: Vec<i64>) -> HashMap<Point, Tile> {
    let mut machine = intcode::create_new_machine(program);
    let mut map: HashMap<Point, Tile> = HashMap::new();

    map.insert((0, 0), Tile::Empty);

    visit(&mut machine, &mut map, (0, 0));

    return map;
}

fn find_path_length(map: &HashMap<Point, Tile>, start: Point) -> i32 {
    let mut to_visit: VecDeque<Point> = VecDeque::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();
    let oxygen_pos = map.iter().find(|item| *item.1 == Tile::Oxygen).unwrap().0;

    to_visit.push_back(start);

    while let Some(pos) = to_visit.pop_front() {
        if pos == *oxygen_pos {
            break;
        }
        for dir in 1..5 {
            let new_pos = get_new_position(dir, &pos);
            if map.contains_key(&new_pos)
                && *map.get(&new_pos).unwrap() != Tile::Wall
                && !prev.contains_key(&new_pos)
            {
                prev.insert(new_pos, pos);
                to_visit.push_front(new_pos);
            }
        }
    }

    let mut backtrack = *oxygen_pos;
    let mut length = 0;

    while let Some(node) = prev.get(&backtrack) {
        length += 1;
        if *node == start {
            break;
        }

        backtrack = *node;
    }

    return length;
}

fn solve(map: HashMap<Point, Tile>) {
    let mut max = 0;

    for entry in &map {
        let (point, tile) = entry;

        if *tile != Tile::Wall {
            let length = find_path_length(&map, *point);
            if length > max {
                max = length;
            }
        }
    }

    println!("{}", max);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let input = tokenize(filepath);
    let map = generate_map(input);
    solve(map);
}
