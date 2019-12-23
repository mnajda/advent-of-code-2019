use std::env;
use std::fs;

mod intcode;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn tokenize(filepath: &String) -> Vec<i64> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn get_direction(new_move: i64, last_move: &Direction) -> Direction {
    match last_move {
        Direction::Up => {
            if new_move == 1 {
                return Direction::Right;
            } else {
                return Direction::Left;
            }
        }
        Direction::Down => {
            if new_move == 1 {
                return Direction::Left;
            } else {
                return Direction::Right;
            }
        }
        Direction::Left => {
            if new_move == 1 {
                return Direction::Up;
            } else {
                return Direction::Down;
            }
        }
        Direction::Right => {
            if new_move == 1 {
                return Direction::Down;
            } else {
                return Direction::Up;
            }
        }
    }
}

fn get_position(last_position: &(i64, i64), direction: &Direction) -> (i64, i64) {
    match direction {
        Direction::Up => (last_position.0, last_position.1 + 1),
        Direction::Down => (last_position.0, last_position.1 - 1),
        Direction::Left => (last_position.0 - 1, last_position.1),
        Direction::Right => (last_position.0 + 1, last_position.1),
    }
}

fn get_output(output: Option<i64>) -> i64 {
    match output {
        Some(item) => item,
        None => panic!("No output value"),
    }
}

fn is_unique(pos: &(i64, i64), moves: &Vec<(i64, (i64, i64))>) -> bool {
    return !moves.iter().any(|item| item.1 == *pos);
}

fn get_current_color(pos: &(i64, i64), moves: &Vec<(i64, (i64, i64))>) -> i64 {
    match moves.iter().find(|item| item.1 == *pos) {
        Some(item) => item.0,
        None => 0,
    }
}

fn get_colored_map(program: Vec<i64>) -> Vec<(i64, (i64, i64))> {
    let mut moves: Vec<(i64, (i64, i64))> = Vec::new();
    let mut machine = intcode::create_new_machine(program);
    let mut position: (i64, i64) = (0, 0);
    let mut direction: Direction = Direction::Up;

    moves.push((1, position));

    while !machine.is_finished() {
        let input = get_current_color(&position, &moves);

        machine.push_input(input);
        machine.execute_program();

        let color = get_output(machine.pop_output());
        let next_move = get_output(machine.pop_output());

        if is_unique(&position, &moves) {
            moves.push((color, position));
        } else {
            let elem = moves.iter().position(|item| item.1 == position).unwrap();
            moves[elem] = (color, position);
        }

        direction = get_direction(next_move, &direction);
        position = get_position(&position, &direction);
    }

    return moves;
}

fn scale(map: Vec<(i64, (i64, i64))>) -> Vec<(i64, (i64, i64))> {
    let min_x = (map
        .iter()
        .min_by(|lhs, rhs| (lhs.1).0.cmp(&(rhs.1).0))
        .unwrap()
        .1)
        .0;
    let min_y = (map
        .iter()
        .min_by(|lhs, rhs| (lhs.1).1.cmp(&(rhs.1).1))
        .unwrap()
        .1)
        .1;

    let offset_x = -(min_x);
    let offset_y = -(min_y);

    return map
        .iter()
        .map(|item| (item.0, ((item.1).0 + offset_x, (item.1).1 + offset_y)))
        .collect();
}

fn get_printable_map(map: Vec<(i64, (i64, i64))>) -> Vec<Vec<char>> {
    let max_x = (map
        .iter()
        .max_by(|lhs, rhs| (lhs.1).0.cmp(&(rhs.1).0))
        .unwrap()
        .1)
        .0;
    let max_y = (map
        .iter()
        .max_by(|lhs, rhs| (lhs.1).1.cmp(&(rhs.1).1))
        .unwrap()
        .1)
        .1;
    let mut printable: Vec<Vec<char>> = vec![vec![' '; max_y as usize + 1]; max_x as usize + 1];

    for item in map {
        let color = item.0;
        let (x, y) = item.1;

        match color {
            0 => printable[x as usize][y as usize] = ' ',
            1 => printable[x as usize][y as usize] = '#',
            _ => panic!("Unexpected color"),
        }
    }

    return printable;
}

fn print(map: Vec<Vec<char>>) {
    let mut image: String = String::new();

    for row in map {
        let temp: String = row.iter().map(|item| item.to_string()).collect();
        image.push_str(&temp);
        image.push_str("\n");
    }

    println!("{}", image);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];

    let program = tokenize(filepath);
    let map = get_colored_map(program);
    let scaled = scale(map);
    let printable = get_printable_map(scaled);
    print(printable);
}
