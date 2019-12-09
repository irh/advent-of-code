// https://adventofcode.com/2019/day/9

use std::{collections::HashMap, iter::FromIterator};

type Address = u64;
type Value = i64;

fn get_digits(number: Value) -> Vec<u8> {
    assert!(number >= 0);

    if number < 10 {
        return vec![number as u8];
    }

    let count = (number as f64).log10().floor() as usize + 1;
    let mut digits = Vec::with_capacity(count);
    let mut x = number;
    while x > 0 {
        digits.insert(0, (x % 10) as u8);
        x /= 10;
    }

    digits
}

enum ParameterMode {
    Position,
    Relative,
    Immediate,
}

#[derive(Default)]
struct Program {
    state: HashMap<Address, Value>,
    ip: Address,
    relative_base: Value,
    inputs: Vec<Value>,
    current_input: usize,
}

impl Program {
    fn new(input: &str) -> Self {
        Self {
            state: HashMap::from_iter(input.split(',').enumerate().map(|(i, x)| {
                (
                    i as Address,
                    x.trim()
                        .parse::<Value>()
                        .expect(&format!("Unable to parse program value: '{}'", x)),
                )
            })),
            ..Default::default()
        }
    }

    fn set_input(&mut self, input: &[Value]) {
        self.inputs = input.to_vec();
        self.current_input = 0;
    }

    fn read(&mut self, position: Address) -> Value {
        *self.state.get(&position).unwrap_or(&0)
    }

    fn write(&mut self, position: Address, value: Value) {
        self.state.insert(position, value);
    }

    fn parameter_mode(&self, id: usize, digits: &Vec<u8>) -> ParameterMode {
        let mode_offset = 2 + id;
        if digits.len() < mode_offset {
            return ParameterMode::Position;
        }
        match digits[digits.len() - mode_offset] {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unexpected parameter mode"),
        }
    }

    fn read_parameter(&mut self, id: usize, digits: &Vec<u8>) -> Value {
        let value = self.read(self.ip + id as Address);
        match self.parameter_mode(id, digits) {
            ParameterMode::Position => return self.read(value as Address),
            ParameterMode::Relative => return self.read((self.relative_base + value) as Address),
            ParameterMode::Immediate => return value,
        }
    }

    fn write_parameter(&mut self, id: usize, digits: &Vec<u8>, value: Value) {
        let position = self.read(self.ip + id as Address);
        match self.parameter_mode(id, digits) {
            ParameterMode::Position => self.write(position as Address, value),
            ParameterMode::Relative => {
                self.write((self.relative_base + position) as Address, value)
            }
            ParameterMode::Immediate => panic!("Unexpected parameter mode for write"),
        }
    }

    fn run(&mut self) -> Vec<Value> {
        self.collect()
    }
}

impl Iterator for Program {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let op = self.read(self.ip);
            let d = get_digits(op);

            match &d[(d.len() - d.len().min(2))..] {
                [1] | [0, 1] => {
                    // Sum
                    let a = self.read_parameter(1, &d);
                    let b = self.read_parameter(2, &d);
                    self.write_parameter(3, &d, a + b);
                    self.ip += 4;
                }
                [2] | [0, 2] => {
                    // Multiply
                    let a = self.read_parameter(1, &d);
                    let b = self.read_parameter(2, &d);
                    self.write_parameter(3, &d, a * b);
                    self.ip += 4;
                }
                [3] | [0, 3] => {
                    // Store input
                    self.write_parameter(1, &d, self.inputs[self.current_input]);
                    self.current_input += 1;
                    self.ip += 2;
                }
                [4] | [0, 4] => {
                    // Output
                    let value = self.read_parameter(1, &d);
                    self.ip += 2;
                    return Some(value);
                }
                [5] | [0, 5] => {
                    // jump-if-true
                    let a = self.read_parameter(1, &d);
                    let b = self.read_parameter(2, &d);
                    if a != 0 {
                        self.ip = b as Address;
                    } else {
                        self.ip += 3;
                    }
                }
                [6] | [0, 6] => {
                    // jump-if-false
                    let a = self.read_parameter(1, &d);
                    let b = self.read_parameter(2, &d);
                    if a == 0 {
                        self.ip = b as Address;
                    } else {
                        self.ip += 3;
                    }
                }
                [7] | [0, 7] => {
                    // Less than
                    let a = self.read_parameter(1, &d);
                    let b = self.read_parameter(2, &d);
                    self.write_parameter(3, &d, if a < b { 1 } else { 0 });
                    self.ip += 4;
                }
                [8] | [0, 8] => {
                    // Equals
                    let a = self.read_parameter(1, &d);
                    let b = self.read_parameter(2, &d);
                    self.write_parameter(3, &d, if a == b { 1 } else { 0 });
                    self.ip += 4;
                }
                [9] | [0, 9] => {
                    // Relative base offset
                    let a = self.read_parameter(1, &d);
                    self.relative_base += a;
                    self.ip += 2;
                }
                [9, 9] => {
                    // Halt
                    return None;
                }
                invalid => panic!("Invalid opcode {:?} at position {}", invalid, self.ip),
            }
        }
    }
}

fn main() {
    let input = include_str!("input/9");

    {
        let mut program = Program::new(input);
        program.set_input(&[1]);
        let result = program.run();
        println!("BOOST keycode {}", result[0]);
    }

    {
        let mut program = Program::new(input);
        program.set_input(&[2]);
        let result = program.run();
        println!("Distress signal coordinates {}", result[0]);
    }
}

#[cfg(test)]
mod day_9 {
    use super::*;

    #[test]
    fn test_0() {
        let mut program = Program::new("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        assert_eq!(
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            program.run()
        );
    }

    #[test]
    fn test_1() {
        let mut program = Program::new("1102,34915192,34915192,7,4,7,99,0");
        let result = program.run()[0];
        assert!(result >= 1_000_000_000_000_000);
    }

    #[test]
    fn test_2() {
        let mut program = Program::new("104,1125899906842624,99");
        assert_eq!(vec![1125899906842624], program.run());
    }
}
