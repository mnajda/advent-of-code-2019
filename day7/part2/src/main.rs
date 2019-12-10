use permutohedron::LexicalPermutation;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn get_operation_size(op: &i32) -> usize {
    match op {
        1 | 2 | 7 | 8 => 4,
        3 | 4 => 2,
        5 | 6 => 3,
        _ => panic!("Unexpected operation"),
    }
}

fn get_addressing(input: i32, op_size: usize) -> Vec<i32> {
    let mut temp = input;
    let mut addressing: Vec<i32> = Vec::new();

    for _ in 0..op_size - 1 {
        addressing.push(temp % 10);
        temp /= 10;
    }

    return addressing;
}

fn get_value(addressing: i32, pos: usize, program: &Vec<i32>) -> i32 {
    if addressing == 0 {
        return program[program[pos] as usize];
    } else {
        return program[pos];
    }
}

struct Amp {
    program: Vec<i32>,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    is_stopped: bool,
    is_finished: bool,
    ip: usize,
}

impl Amp {
    fn execute_program(&mut self) {
        while self.ip < self.program.len() && !self.is_stopped {
            if self.program[self.ip] == 99 {
                self.is_finished = true;
                break;
            }
            let op = self.program[self.ip] % 100;
            let op_size = get_operation_size(&op);
            let addressing_modes = self.program[self.ip] / 100;
            self.execute_operation(op, get_addressing(addressing_modes, op_size));
        }
    }

    fn execute_operation(&mut self, op: i32, addressing: Vec<i32>) {
        // TODO refactor this awful function
        match op {
            1 => {
                let lhs = get_value(addressing[0], self.ip + 1, &self.program);
                let rhs = get_value(addressing[1], self.ip + 2, &self.program);
                let dest = self.program[self.ip + 3] as usize;

                self.program[dest] = lhs + rhs;
                self.ip += get_operation_size(&op);
            }
            2 => {
                let lhs = get_value(addressing[0], self.ip + 1, &self.program);
                let rhs = get_value(addressing[1], self.ip + 2, &self.program);
                let dest = self.program[self.ip + 3] as usize;

                self.program[dest] = lhs * rhs;
                self.ip += get_operation_size(&op);
            }
            3 => {
                let dest = self.program[self.ip + 1] as usize;
                match self.input.pop_front() {
                    Some(item) => {
                        self.program[dest] = item;
                        self.ip += get_operation_size(&op);
                    }
                    None => self.is_stopped = true,
                };
            }
            4 => {
                let source = get_value(addressing[0], self.ip + 1, &self.program) as usize;
                self.output.push_back(source as i32);

                self.ip += get_operation_size(&op);
            }
            5 => {
                let condition = get_value(addressing[0], self.ip + 1, &self.program);
                let dest = get_value(addressing[1], self.ip + 2, &self.program);

                if condition != 0 {
                    self.ip = dest as usize;
                } else {
                    self.ip += get_operation_size(&op);
                }
            }
            6 => {
                let condition = get_value(addressing[0], self.ip + 1, &self.program);
                let dest = get_value(addressing[1], self.ip + 2, &self.program);

                if condition == 0 {
                    self.ip = dest as usize;
                } else {
                    self.ip += get_operation_size(&op);
                }
            }
            7 => {
                let lhs = get_value(addressing[0], self.ip + 1, &self.program);
                let rhs = get_value(addressing[1], self.ip + 2, &self.program);
                let dest = self.program[self.ip + 3] as usize;

                if lhs < rhs {
                    self.program[dest] = 1;
                } else {
                    self.program[dest] = 0;
                }
                self.ip += get_operation_size(&op);
            }
            8 => {
                let lhs = get_value(addressing[0], self.ip + 1, &self.program);
                let rhs = get_value(addressing[1], self.ip + 2, &self.program);
                let dest = self.program[self.ip + 3] as usize;

                if lhs == rhs {
                    self.program[dest] = 1;
                } else {
                    self.program[dest] = 0;
                }
                self.ip += get_operation_size(&op);
            }
            _ => {
                panic!("Unexpected operation");
            }
        }
    }
}

fn tokenize(filepath: &String) -> Vec<i32> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn create_amps(phases: &[i32], program: &Vec<i32>) -> Vec<Amp> {
    let mut amps: Vec<Amp> = Vec::new();

    amps.push(Amp {
        program: program.clone(),
        input: VecDeque::from(vec![phases[0], 0]),
        output: VecDeque::new(),
        is_stopped: false,
        is_finished: false,
        ip: 0,
    });
    amps.push(Amp {
        program: program.clone(),
        input: VecDeque::from(vec![phases[1]]),
        output: VecDeque::new(),
        is_stopped: false,
        is_finished: false,
        ip: 0,
    });
    amps.push(Amp {
        program: program.clone(),
        input: VecDeque::from(vec![phases[2]]),
        output: VecDeque::new(),
        is_stopped: false,
        is_finished: false,
        ip: 0,
    });
    amps.push(Amp {
        program: program.clone(),
        input: VecDeque::from(vec![phases[3]]),
        output: VecDeque::new(),
        is_stopped: false,
        is_finished: false,
        ip: 0,
    });
    amps.push(Amp {
        program: program.clone(),
        input: VecDeque::from(vec![phases[4]]),
        output: VecDeque::new(),
        is_stopped: false,
        is_finished: false,
        ip: 0,
    });

    return amps;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let mut max: i32 = 0;
    let mut phases = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();

    let program = tokenize(filepath);

    loop {
        permutations.push(phases.to_vec());
        if !phases.next_permutation() {
            break;
        }
        let mut amps: Vec<Amp> = create_amps(&phases, &program);

        loop {
            for i in 0..5 {
                amps[i].execute_program();

                let output = amps[i].output.pop_front().unwrap();
                if i == 4 {
                    amps[0].input.push_back(output);
                    amps[i].is_stopped = false;
                    if output > max {
                        max = output;
                    }
                    continue;
                }
                amps[i + 1].input.push_back(output);
                amps[i].is_stopped = false;
            }
            if amps[4].is_finished {
                break;
            }
        }
    }
    println!("{}", max);
}
