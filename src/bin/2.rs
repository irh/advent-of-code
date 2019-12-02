// https://adventofcode.com/2019/day/2

type Opcode = usize;
struct Program(Vec<Opcode>);

impl Program {
    fn new(input: &str) -> Self {
        Self(
            input
                .split(',')
                .map(|x| {
                    x.trim()
                        .parse::<Opcode>()
                        .expect(&format!("Unable to parse opcode: '{}'", x))
                })
                .collect(),
        )
    }

    fn new_with_noun_verb(input: &str, noun: Opcode, verb: Opcode) -> Self {
        let mut program = Self::new(input);
        program.0[1] = noun;
        program.0[2] = verb;
        program
    }

    fn run(&mut self) {
        let mut x = 0;

        loop {
            let opcode = self.0[x];
            match opcode {
                1 => {
                    let a = self.0[x + 1];
                    let b = self.0[x + 2];
                    let destination = self.0[x + 3];
                    self.0[destination] = self.0[a] + self.0[b];
                    x += 4;
                }
                2 => {
                    let a = self.0[x + 1];
                    let b = self.0[x + 2];
                    let destination = self.0[x + 3];
                    self.0[destination] = self.0[a] * self.0[b];
                    x += 4;
                }
                99 => return,
                _ => panic!("Invalid opcode {} at position {}", opcode, x),
            }
        }
    }
}

fn main() {
    let input = include_str!("input/2");
    let mut program = Program::new_with_noun_verb(&input, 12, 2);
    program.run();

    // Part one
    println!("1202 program alarm result: {}", program.0[0]);

    // Part two
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = Program::new_with_noun_verb(&input, noun, verb);
            program.run();
            if program.0[0] == 19690720 {
                println!(
                    "Program resulting in 19690720 - noun: {} verb: {} - result: {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return;
            }
        }
    }
}

#[cfg(test)]
mod day_2 {
    use super::*;

    #[test]
    fn test_0() {
        let mut program = Program::new("1,0,0,0,99");
        program.run();
        assert_eq!(2, program.0[0]);
    }

    #[test]
    fn test_1() {
        let mut program = Program::new("2,3,0,3,99");
        program.run();
        assert_eq!(6, program.0[3]);
    }

    #[test]
    fn test_2() {
        let mut program = Program::new("2,4,4,5,99,0");
        program.run();
        assert_eq!(9801, program.0[5]);
    }

    #[test]
    fn test_3() {
        let mut program = Program::new("1,1,1,4,99,5,6,0,99");
        program.run();
        assert_eq!(30, program.0[0]);
        assert_eq!(2, program.0[4]);
    }
}
