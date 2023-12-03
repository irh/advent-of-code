// https://adventofcode.com/2019/day/7

type Value = i32;

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

#[derive(Default)]
struct Program {
    state: Vec<Value>,
    ip: usize,
    inputs: Vec<Value>,
    current_input: usize,
}

impl Program {
    fn new(input: &str) -> Self {
        Self {
            state: input
                .split(',')
                .map(|x| {
                    x.trim()
                        .parse::<Value>()
                        .unwrap_or_else(|_| panic!("Unable to parse program value: '{x}'"))
                })
                .collect(),
            ..Default::default()
        }
    }

    fn reset(&mut self, program: &Program) {
        self.state.clear();
        self.state.extend_from_slice(program.state.as_slice());
        self.inputs.clear();
        self.ip = 0;
        self.current_input = 0;
    }

    fn set_input(&mut self, input: &[Value]) {
        self.inputs = input.to_vec();
        self.current_input = 0;
    }

    fn add_to_input(&mut self, input: &[Value]) {
        self.inputs.extend_from_slice(input)
    }

    fn get_parameter(&self, position: usize, id: usize, digits: &Vec<u8>) -> Value {
        let value = self.state[position + id];
        let mode_offset = 2 + id;
        if digits.len() < mode_offset {
            return self.state[value as usize];
        }
        match digits[digits.len() - mode_offset] {
            0 => self.state[value as usize],
            1 => value,
            _ => panic!("Unexpected parameter mode"),
        }
    }
}

impl Iterator for Program {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let d = get_digits(self.state[self.ip]);

            match &d[(d.len() - d.len().min(2))..] {
                [1] | [0, 1] => {
                    // Sum
                    let a = self.get_parameter(self.ip, 1, &d);
                    let b = self.get_parameter(self.ip, 2, &d);
                    let destination = self.state[self.ip + 3] as usize;
                    self.state[destination] = a + b;
                    self.ip += 4;
                }
                [2] | [0, 2] => {
                    // Multiply
                    let a = self.get_parameter(self.ip, 1, &d);
                    let b = self.get_parameter(self.ip, 2, &d);
                    let destination = self.state[self.ip + 3] as usize;
                    self.state[destination] = a * b;
                    self.ip += 4;
                }
                [3] | [0, 3] => {
                    // Store input
                    let destination = self.state[self.ip + 1] as usize;
                    self.state[destination] = self.inputs[self.current_input];
                    self.current_input += 1;
                    self.ip += 2;
                }
                [4] | [0, 4] => {
                    // Output
                    let value = self.get_parameter(self.ip, 1, &d);
                    self.ip += 2;
                    return Some(value);
                }
                [5] | [0, 5] => {
                    // jump-if-true
                    let a = self.get_parameter(self.ip, 1, &d);
                    let b = self.get_parameter(self.ip, 2, &d);
                    if a != 0 {
                        self.ip = b as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                [6] | [0, 6] => {
                    // jump-if-false
                    let a = self.get_parameter(self.ip, 1, &d);
                    let b = self.get_parameter(self.ip, 2, &d);
                    if a == 0 {
                        self.ip = b as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                [7] | [0, 7] => {
                    // Less than
                    let a = self.get_parameter(self.ip, 1, &d);
                    let b = self.get_parameter(self.ip, 2, &d);
                    let destination = self.state[self.ip + 3] as usize;
                    self.state[destination] = if a < b { 1 } else { 0 };
                    self.ip += 4;
                }
                [8] | [0, 8] => {
                    // Equals
                    let a = self.get_parameter(self.ip, 1, &d);
                    let b = self.get_parameter(self.ip, 2, &d);
                    let destination = self.state[self.ip + 3] as usize;
                    self.state[destination] = if a == b { 1 } else { 0 };
                    self.ip += 4;
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

struct AmpCircuit {
    amps: [Program; 5],
    program: Program,
}

impl AmpCircuit {
    fn new(input: &str) -> Self {
        let mut circuit = Self {
            amps: Default::default(),
            program: Program::new(input),
        };
        circuit.reset();
        circuit
    }

    fn reset(&mut self) {
        for amp in self.amps.iter_mut() {
            amp.reset(&self.program);
        }
    }

    fn run(&mut self, phases: &[Value; 5], allow_feedback: bool) -> Option<Value> {
        let mut previous_output = Some(0);
        let mut result = None;

        for (amp, &phase) in self.amps.iter_mut().zip(phases) {
            amp.set_input(&[phase]);
        }

        loop {
            let outputs: Vec<Option<Value>> = self
                .amps
                .iter_mut()
                .map(|amp| {
                    if let Some(input) = previous_output {
                        amp.add_to_input(&[input]);
                    }
                    let amp_output = amp.next();
                    previous_output = amp_output;
                    amp_output
                })
                .collect();

            if outputs.last().unwrap().is_some() {
                result = *outputs.last().unwrap();
            }

            if !allow_feedback || outputs.iter().all(|amp_output| amp_output.is_none()) {
                return result;
            }
        }
    }
}

fn main() {
    let mut circuit = AmpCircuit::new(include_str!("input/7"));

    let mut max_signal = 0;
    for phases in permutohedron::Heap::new(&mut [0, 1, 2, 3, 4]) {
        circuit.reset();
        let signal = circuit.run(&phases, false).unwrap();
        if signal > max_signal {
            max_signal = signal;
        }
    }
    println!("Max signal without feedback: {}", max_signal);

    let mut max_signal = 0;
    for phases in permutohedron::Heap::new(&mut [5, 6, 7, 8, 9]) {
        circuit.reset();
        let signal = circuit.run(&phases, true).unwrap();
        if signal > max_signal {
            max_signal = signal;
        }
    }
    println!("Max signal with feedback: {}", max_signal);
}

#[cfg(test)]
mod day_7 {
    use super::*;

    #[test]
    fn test_0() {
        let mut circuit = AmpCircuit::new("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(Some(43210), circuit.run(&[4, 3, 2, 1, 0], false));
    }

    #[test]
    fn test_1() {
        let mut circuit = AmpCircuit::new(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,
101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(Some(54321), circuit.run(&[0, 1, 2, 3, 4], false));
    }

    #[test]
    fn test_2() {
        let mut circuit = AmpCircuit::new(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        assert_eq!(Some(65210), circuit.run(&[1, 0, 4, 3, 2], false));
    }

    #[test]
    fn test_3() {
        let mut circuit = AmpCircuit::new(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(Some(139629729), circuit.run(&[9, 8, 7, 6, 5], true));
    }

    #[test]
    fn test_4() {
        let mut circuit = AmpCircuit::new(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(Some(18216), circuit.run(&[9, 7, 8, 5, 6], true));
    }
}
