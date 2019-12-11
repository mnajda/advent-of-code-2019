use std::env;
use std::fs;

fn tokenize(filepath: &String) -> Vec<i64> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn get_operation_size(op: &i64) -> usize {
    match op {
        1 | 2 | 7 | 8 => 4,
        3 | 4 | 9 => 2,
        5 | 6 => 3,
        _ => panic!("Unexpected operation"),
    }
}

fn get_addressing(input: i64, op_size: usize) -> Vec<i64> {
    let mut temp = input;
    let mut addressing: Vec<i64> = Vec::new();

    for _ in 0..op_size - 1 {
        addressing.push(temp % 10);
        temp /= 10;
    }

    return addressing;
}

struct Machine {
    program: Vec<i64>,
    ip: usize,
    relative_base: i64,
    input: i64,
}

impl Machine {
    fn get_value(&mut self, addressing: i64, pos: usize) -> i64 {
        match addressing {
            0 => {
                let source = self.program[pos] as usize;

                self.memory_check(source);
                self.program[source]
            }
            1 => self.program[pos],
            2 => {
                let source = (self.relative_base + self.program[pos]) as usize;

                self.memory_check(source);
                self.program[source]
            }
            _ => panic!("Unexpected addressing"),
        }
    }

    fn get_index(&mut self, addressing: i64, pos: usize) -> usize {
        match addressing {
            0 => {
                let source = self.program[pos] as usize;

                self.memory_check(source);
                source
            }
            1 => pos,
            2 => {
                let source = (self.relative_base + self.program[pos]) as usize;

                self.memory_check(source);
                source
            }
            _ => panic!("Unexpected addressing"),
        }
    }

    fn memory_check(&mut self, dest: usize) {
        if dest >= self.program.len() {
            self.program.resize(dest + 1, 0);
        }
    }

    fn write_to_memory(&mut self, dest: usize, input: i64) {
        self.memory_check(dest);

        self.program[dest] = input;
    }

    fn execute_operation(&mut self, op: i64, addressing: Vec<i64>) {
        match op {
            1 => {
                let lhs = self.get_value(addressing[0], self.ip + 1);
                let rhs = self.get_value(addressing[1], self.ip + 2);
                let dest = self.get_index(addressing[2], self.ip + 3);

                self.write_to_memory(dest, lhs + rhs);
                self.ip += get_operation_size(&op);
            }
            2 => {
                let lhs = self.get_value(addressing[0], self.ip + 1);
                let rhs = self.get_value(addressing[1], self.ip + 2);
                let dest = self.get_index(addressing[2], self.ip + 3);

                self.write_to_memory(dest, lhs * rhs);
                self.ip += get_operation_size(&op);
            }
            3 => {
                let dest = self.get_index(addressing[0], self.ip + 1);

                self.write_to_memory(dest, self.input);
                self.ip += get_operation_size(&op);
            }
            4 => {
                let source = self.get_value(addressing[0], self.ip + 1);

                println!("{}", source);
                self.ip += get_operation_size(&op);
            }
            5 => {
                let condition = self.get_value(addressing[0], self.ip + 1);
                let dest = self.get_value(addressing[1], self.ip + 2);

                if condition != 0 {
                    self.ip = dest as usize;
                } else {
                    self.ip += get_operation_size(&op);
                }
            }
            6 => {
                let condition = self.get_value(addressing[0], self.ip + 1);
                let dest = self.get_value(addressing[1], self.ip + 2);

                if condition == 0 {
                    self.ip = dest as usize;
                } else {
                    self.ip += get_operation_size(&op);
                }
            }
            7 => {
                let lhs = self.get_value(addressing[0], self.ip + 1);
                let rhs = self.get_value(addressing[1], self.ip + 2);
                let dest = self.get_index(addressing[2], self.ip + 3);

                if lhs < rhs {
                    self.write_to_memory(dest, 1);
                } else {
                    self.write_to_memory(dest, 0);
                }
                self.ip += get_operation_size(&op);
            }
            8 => {
                let lhs = self.get_value(addressing[0], self.ip + 1);
                let rhs = self.get_value(addressing[1], self.ip + 2);
                let dest = self.get_index(addressing[2], self.ip + 3);

                if lhs == rhs {
                    self.write_to_memory(dest, 1);
                } else {
                    self.write_to_memory(dest, 0);
                }
                self.ip += get_operation_size(&op);
            }
            9 => {
                let adjustment = self.get_value(addressing[0], self.ip + 1);
                self.relative_base += adjustment;

                self.ip += get_operation_size(&op);
            }
            _ => {
                panic!("Unexpected operation");
            }
        }
    }

    fn execute_program(&mut self) {
        while self.ip < self.program.len() {
            if self.program[self.ip] == 99 {
                break;
            }
            let op = self.program[self.ip] % 100;
            let op_size = get_operation_size(&op);
            let addressing_modes = self.program[self.ip] / 100;
            self.execute_operation(op, get_addressing(addressing_modes, op_size));
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Provide filename and input");
        return;
    }
    let filepath = &args[1];
    let input: i64 = args[2].parse().unwrap();

    let program = tokenize(filepath);
    let mut machine = Machine {
        program: program,
        ip: 0,
        relative_base: 0,
        input: input,
    };
    machine.execute_program();
}
