// https://adventofcode.com/2019/day/11

mod intcode;
use intcode::Program;

use std::{collections::HashMap, fmt};

type Position = (i32, i32);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Painter {
    program: Program,
    panels: HashMap<Position, u8>,
    position: Position,
    direction: Direction,
}

impl Painter {
    fn new(input: &str, initial_color: u8) -> Self {
        let position = (0, 0);
        let mut panels = HashMap::new();
        panels.insert(position, initial_color);

        Self {
            program: Program::new(input),
            panels,
            position,
            direction: Direction::Up,
        }
    }

    fn paint(&mut self) {
        use Direction::*;

        loop {
            let panel_color = *self.panels.entry(self.position).or_insert(0);
            self.program.set_input(&[panel_color as intcode::Value]);
            if let Some(color) = self.program.next() {
                if let Some(turn) = self.program.next() {
                    self.panels.insert(self.position, color as u8);

                    self.direction = match (turn, self.direction) {
                        (0, Up) => Left,
                        (0, Right) => Up,
                        (0, Down) => Right,
                        (0, Left) => Down,
                        (1, Up) => Right,
                        (1, Right) => Down,
                        (1, Down) => Left,
                        (1, Left) => Up,
                        _ => panic!("Unexpected turn instruction"),
                    };
                    match self.direction {
                        Up => self.position.1 += 1,
                        Right => self.position.0 += 1,
                        Down => self.position.1 -= 1,
                        Left => self.position.0 -= 1,
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

impl fmt::Display for Painter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;

        for p in self.panels.keys() {
            x_min = x_min.min(p.0);
            x_max = x_max.max(p.0);
            y_min = y_min.min(p.1);
            y_max = y_max.max(p.1);
        }

        for y in 0..=(y_max - y_min) {
            for x in 0..=(x_max - x_min) {
                match self.panels.get(&(x + x_min, y_max - y)) {
                    Some(1) => write!(f, "█")?,
                    _ => write!(f, "░")?,
                };
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("input/11");
    {
        let mut painter = Painter::new(input, 0);
        painter.paint();
        println!(
            "Number of panels painted when starting on a black panel: {}",
            painter.panels.len()
        );
    }
    {
        let mut painter = Painter::new(input, 1);
        painter.paint();
        println!(
            "Result of painting after starting with white panel:\n{}",
            painter
        );
    }
}
