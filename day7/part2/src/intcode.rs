use std::collections::VecDeque;

pub struct Machine {
    program: Vec<i64>,
    ip: usize,
    relative_base: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    is_halted: bool,
    is_finished: bool,
}

pub fn create_new_machine(program: Vec<i64>) -> Machine {
    return Machine {
        program: program,
        ip: 0,
        relative_base: 0,
        input: VecDeque::<i64>::new(),
        output: VecDeque::<i64>::new(),
        is_halted: false,
        is_finished: false,
    }
}

impl Machine {
    fn get_address(&mut self, addressing: i64, pos: usize) -> usize {
        match addressing {
            0 => {
                let address = self.program[pos] as usize;

                self.memory_check(address);
                return address;
            }
            1 => return pos,
            2 => {
                let address = (self.relative_base + self.program[pos]) as usize;

                self.memory_check(address);
                return address;
            }
            _ => panic!("Unexpected addressing"),
        }
    }

    fn get_value(&mut self, addressing: i64, pos: usize) -> i64 {
        let address = self.get_address(addressing, pos);

        return self.program[address];
    }

    fn memory_check(&mut self, address: usize) {
        if address >= self.program.len() {
            self.program.resize(address + 1, 0);
        }
    }

    fn write_to_memory(&mut self, address: usize, input: i64) {
        self.memory_check(address);

        self.program[address] = input;
    }

    fn get_operation_size(op: &i64) -> usize {
        match op {
            1 | 2 | 7 | 8 => 4,
            3 | 4 | 9 => 2,
            5 | 6 => 3,
            _ => panic!("Unexpected operation"),
        }
    }

    fn add(&mut self, op: i64, addressing: Vec<i64>) {
        let lhs = self.get_value(addressing[0], self.ip + 1);
        let rhs = self.get_value(addressing[1], self.ip + 2);
        let dest = self.get_address(addressing[2], self.ip + 3);

        self.write_to_memory(dest, lhs + rhs);
        self.ip += Machine::get_operation_size(&op);
    }

    fn multiply(&mut self, op: i64, addressing: Vec<i64>) {
        let lhs = self.get_value(addressing[0], self.ip + 1);
        let rhs = self.get_value(addressing[1], self.ip + 2);
        let dest = self.get_address(addressing[2], self.ip + 3);

        self.write_to_memory(dest, lhs * rhs);
        self.ip += Machine::get_operation_size(&op);
    }

    fn write(&mut self, op: i64, addressing: Vec<i64>) {
        let dest = self.get_address(addressing[0], self.ip + 1);

        match self.input.pop_front() {
            Some(item) => {
                self.write_to_memory(dest, item);
                self.ip += Machine::get_operation_size(&op);
            }
            None => self.is_halted = true,
        }
    }

    fn read(&mut self, op: i64, addressing: Vec<i64>) {
        let value = self.get_value(addressing[0], self.ip + 1);

        self.output.push_back(value);
        self.ip += Machine::get_operation_size(&op);
    }

    fn jump_if_false(&mut self, op: i64, addressing: Vec<i64>) {
        let condition = self.get_value(addressing[0], self.ip + 1);
        let dest = self.get_value(addressing[1], self.ip + 2);

        if condition != 0 {
            self.ip = dest as usize;
        } else {
            self.ip += Machine::get_operation_size(&op);
        }
    }

    fn jump_if_true(&mut self, op: i64, addressing: Vec<i64>) {
        let condition = self.get_value(addressing[0], self.ip + 1);
        let dest = self.get_value(addressing[1], self.ip + 2);

        if condition == 0 {
            self.ip = dest as usize;
        } else {
            self.ip += Machine::get_operation_size(&op);
        }
    }

    fn less_than(&mut self, op: i64, addressing: Vec<i64>) {
        let lhs = self.get_value(addressing[0], self.ip + 1);
        let rhs = self.get_value(addressing[1], self.ip + 2);
        let dest = self.get_address(addressing[2], self.ip + 3);

        if lhs < rhs {
            self.write_to_memory(dest, 1);
        } else {
            self.write_to_memory(dest, 0);
        }
        self.ip += Machine::get_operation_size(&op);
    }

    fn equals(&mut self, op: i64, addressing: Vec<i64>) {
        let lhs = self.get_value(addressing[0], self.ip + 1);
        let rhs = self.get_value(addressing[1], self.ip + 2);
        let dest = self.get_address(addressing[2], self.ip + 3);

        if lhs == rhs {
            self.write_to_memory(dest, 1);
        } else {
            self.write_to_memory(dest, 0);
        }
        self.ip += Machine::get_operation_size(&op);
    }

    fn adjust_relative_base(&mut self, op: i64, addressing: Vec<i64>) {
        let adjustment = self.get_value(addressing[0], self.ip + 1);
        self.relative_base += adjustment;

        self.ip += Machine::get_operation_size(&op);
    }

    fn execute_operation(&mut self, op: i64, addressing: Vec<i64>) {
        match op {
            1 => self.add(op, addressing),
            2 => self.multiply(op, addressing),
            3 => self.write(op, addressing),
            4 => self.read(op, addressing),
            5 => self.jump_if_false(op, addressing),
            6 => self.jump_if_true(op, addressing),
            7 => self.less_than(op, addressing),
            8 => self.equals(op, addressing),
            9 => self.adjust_relative_base(op, addressing),
            _ => {
                panic!("Unexpected operation");
            }
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

    pub fn execute_program(&mut self) {
        while self.ip < self.program.len() && !self.is_halted {
            if self.program[self.ip] == 99 {
                self.is_finished = true;
                break;
            }
            let op = self.program[self.ip] % 100;
            let op_size = Machine::get_operation_size(&op);
            let addressing_modes = self.program[self.ip] / 100;

            self.execute_operation(op, Machine::get_addressing(addressing_modes, op_size));
        }
    }

    pub fn push_input(&mut self, input: i64) {
        self.input.push_back(input);
        self.is_halted = false;
    }

    pub fn pop_output(&mut self) -> Option<i64> {
        return self.output.pop_front();
    }

    pub fn is_finished(&self) -> bool {
        return self.is_finished;
    }
}
