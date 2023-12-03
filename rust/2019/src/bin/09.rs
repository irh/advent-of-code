// https://adventofcode.com/2019/day/9

mod intcode;
use intcode::Program;

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

