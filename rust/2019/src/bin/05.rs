// https://adventofcode.com/2019/day/5

type Value = i32;
struct Program(Vec<Value>);

fn get_digits(number: Value) -> Vec<u8> {
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

impl Program {
    fn new(input: &str) -> Self {
        Self(
            input
                .split(',')
                .map(|x| {
                    x.trim()
                        .parse::<Value>()
                        .unwrap_or_else(|_| panic!("Unable to parse program value: '{x}'"))
                })
                .collect(),
        )
    }

    fn get_parameter(&self, position: usize, id: usize, digits: &Vec<u8>) -> Value {
        let value = self.0[position + id];
        let mode_offset = 2 + id;
        if digits.len() < mode_offset {
            return self.0[value as usize];
        }
        match digits[digits.len() - mode_offset] {
            0 => self.0[value as usize],
            1 => value,
            _ => panic!("Unexpected parameter mode"),
        }
    }

    fn run(&mut self, input: Value) -> Vec<Value> {
        let mut output = vec![];
        let mut x: usize = 0;

        loop {
            let d = get_digits(self.0[x]);

            match &d[(d.len() - d.len().min(2))..] {
                [1] | [0, 1] => {
                    // Sum
                    let a = self.get_parameter(x, 1, &d);
                    let b = self.get_parameter(x, 2, &d);
                    let destination = self.0[x + 3] as usize;
                    self.0[destination] = a + b;
                    x += 4;
                }
                [2] | [0, 2] => {
                    // Multiply
                    let a = self.get_parameter(x, 1, &d);
                    let b = self.get_parameter(x, 2, &d);
                    let destination = self.0[x + 3] as usize;
                    self.0[destination] = a * b;
                    x += 4;
                }
                [3] | [0, 3] => {
                    // Store input
                    let destination = self.0[x + 1] as usize;
                    self.0[destination] = input;
                    x += 2;
                }
                [4] | [0, 4] => {
                    // Output
                    let value = self.get_parameter(x, 1, &d);
                    output.push(value);
                    x += 2;
                }
                [5] | [0, 5] => {
                    // jump-if-true
                    let a = self.get_parameter(x, 1, &d);
                    let b = self.get_parameter(x, 2, &d);
                    if a != 0 {
                        x = b as usize;
                    } else {
                        x += 3;
                    }
                }
                [6] | [0, 6] => {
                    // jump-if-false
                    let a = self.get_parameter(x, 1, &d);
                    let b = self.get_parameter(x, 2, &d);
                    if a == 0 {
                        x = b as usize;
                    } else {
                        x += 3;
                    }
                }
                [7] | [0, 7] => {
                    // Less than
                    let a = self.get_parameter(x, 1, &d);
                    let b = self.get_parameter(x, 2, &d);
                    let destination = self.0[x + 3] as usize;
                    self.0[destination] = if a < b { 1 } else { 0 };
                    x += 4;
                }
                [8] | [0, 8] => {
                    // Equals
                    let a = self.get_parameter(x, 1, &d);
                    let b = self.get_parameter(x, 2, &d);
                    let destination = self.0[x + 3] as usize;
                    self.0[destination] = if a == b { 1 } else { 0 };
                    x += 4;
                }
                [9, 9] => return output,
                invalid => panic!("Invalid opcode {:?} at position {}", invalid, x),
            }
        }
    }
}

fn main() {
    let input = include_str!("input/5");

    let mut program = Program::new(input);
    let output = program.run(1);
    assert!(output[..output.len() - 1].iter().all(|&x| x == 0));
    println!("Diagnostic code for input 1 - {:?}", output.last());

    let mut program = Program::new(input);
    let output = program.run(5);
    assert!(output[..output.len() - 1].iter().all(|&x| x == 0));
    println!("Diagnostic code for input 5 - {:?}", output.last());
}

#[cfg(test)]
mod day_5 {
    use super::*;

    #[test]
    fn test_0() {
        let mut program = Program::new("1002,4,3,4,33");
        program.run(0);
        assert_eq!(99, program.0[4]);
    }

    #[test]
    fn test_equal_position_mode() {
        let program_input = "3,9,8,9,10,9,4,9,99,-1,8";
        let mut program = Program::new(program_input);
        assert_eq!(vec![0], program.run(7));
        let mut program = Program::new(program_input);
        assert_eq!(vec![1], program.run(8));
        let mut program = Program::new(program_input);
        assert_eq!(vec![0], program.run(9));
    }

    #[test]
    fn test_less_than_immediate_mode() {
        let program_input = "3,3,1107,-1,8,3,4,3,99";
        let mut program = Program::new(program_input);
        assert_eq!(vec![1], program.run(7));
        let mut program = Program::new(program_input);
        assert_eq!(vec![0], program.run(8));
        let mut program = Program::new(program_input);
        assert_eq!(vec![0], program.run(9));
    }

    #[test]
    fn test_jump_position_mode() {
        let program_input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let mut program = Program::new(program_input);
        assert_eq!(vec![0], program.run(0));
        let mut program = Program::new(program_input);
        assert_eq!(vec![1], program.run(1));
        let mut program = Program::new(program_input);
        assert_eq!(vec![1], program.run(-1));
    }
}
