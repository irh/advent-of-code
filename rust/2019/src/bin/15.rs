mod intcode;

use {
    intcode::Program,
    std::{collections::HashMap, fmt, thread, time},
};

fn clear_screen() {
    print!("\x1bc");
}

fn hide_cursor() {
    print!("\x1b[?25l");
}

fn show_cursor() {
    print!("\x1b[?25h");
}

fn move_cursor(x: i64, y: i64) {
    print!("\x1b[{};{}f", y, x);
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn position_in_direction(p: Position, d: Direction) -> Position {
    use Direction::*;
    match d {
        North => Position { x: p.x, y: p.y - 1 },
        East => Position { x: p.x + 1, y: p.y },
        South => Position { x: p.x, y: p.y + 1 },
        West => Position { x: p.x - 1, y: p.y },
    }
}

#[derive(PartialEq)]
enum GridState {
    Empty,
    Oxygen,
    ActiveOxygen,
    OxygenSystem,
    Wall,
}

struct Room {
    grid: HashMap<Position, GridState>,
    droid: Option<Position>,
    oxygen_system: Option<Position>,
}

impl Room {
    fn new() -> Self {
        let mut grid = HashMap::new();
        let droid = Position::default();
        grid.insert(droid, GridState::Empty);
        Self {
            grid,
            droid: Some(droid),
            oxygen_system: None,
        }
    }

    fn move_droid(&mut self, p: Position) {
        self.grid.entry(p).or_insert(GridState::Empty);
        self.droid = Some(p);
    }

    fn set_wall(&mut self, p: Position) {
        self.grid.insert(p, GridState::Wall);
    }

    fn set_oxygen_system(&mut self, p: Position) {
        self.grid.insert(p, GridState::OxygenSystem);
        self.oxygen_system = Some(p);
    }

    fn start_filling_room_with_oxygen(&mut self) {
        self.grid
            .insert(self.oxygen_system.unwrap(), GridState::ActiveOxygen);
    }

    fn tick_a_minute(&mut self) {
        use {Direction::*, GridState::*};

        let active_oxygen: Vec<Position> = self
            .grid
            .iter()
            .filter(|(_, state)| **state == ActiveOxygen)
            .map(|(p, _)| *p)
            .collect();

        for p in active_oxygen.iter() {
            self.grid.insert(*p, Oxygen);
            let north = position_in_direction(*p, North);
            if self.grid.get(&north) == Some(&Empty) {
                self.grid.insert(north, ActiveOxygen);
            }
            let east = position_in_direction(*p, East);
            if self.grid.get(&east) == Some(&Empty) {
                self.grid.insert(east, ActiveOxygen);
            }
            let south = position_in_direction(*p, South);
            if self.grid.get(&south) == Some(&Empty) {
                self.grid.insert(south, ActiveOxygen);
            }
            let west = position_in_direction(*p, West);
            if self.grid.get(&west) == Some(&Empty) {
                self.grid.insert(west, ActiveOxygen);
            }
        }
    }

    fn empty_space_count(&self) -> usize {
        self.grid
            .values()
            .filter(|state| **state == GridState::Empty)
            .count()
    }

    fn direction_of_unknown_space(&self, p: Position) -> Option<Direction> {
        use Direction::*;
        if self.grid.get(&position_in_direction(p, North)).is_none() {
            Some(North)
        } else if self.grid.get(&position_in_direction(p, East)).is_none() {
            Some(East)
        } else if self.grid.get(&position_in_direction(p, South)).is_none() {
            Some(South)
        } else if self.grid.get(&position_in_direction(p, West)).is_none() {
            Some(West)
        } else {
            None
        }
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        hide_cursor();
        move_cursor(0, 0);

        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;

        for p in self.grid.keys() {
            x_min = x_min.min(p.x);
            x_max = x_max.max(p.x);
            y_min = y_min.min(p.y);
            y_max = y_max.max(p.y);
        }

        for y in 0..=(y_max - y_min) {
            for x in 0..=(x_max - x_min) {
                let rx = x + x_min;
                let ry = y + y_min;
                if self.droid == Some(Position { x: rx, y: ry }) {
                    write!(f, "D")?;
                } else {
                    use GridState::*;

                    match self.grid.get(&Position { x: rx, y: ry }) {
                        Some(Empty) => write!(f, "▒")?,
                        Some(Oxygen) => write!(f, "O")?,
                        Some(ActiveOxygen) => write!(f, "o")?,
                        Some(OxygenSystem) => write!(f, "X")?,
                        Some(Wall) => write!(f, "█")?,
                        _ => write!(f, " ")?,
                    };
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    use Direction::*;

    clear_screen();
    hide_cursor();

    let mut program = Program::new(include_str!("input/15"));

    let mut room = Room::new();
    let mut direction = North;
    let mut journey_back = Vec::new();
    let mut backtracking = false;
    let mut distance_to_oxygen_system = 0;

    loop {
        program.set_input(&[match direction {
            North => 1,
            East => 4,
            South => 2,
            West => 3,
        }]);

        let target = position_in_direction(room.droid.unwrap(), direction);

        let moved = match program.next() {
            Some(0) => {
                // Hit wall
                room.set_wall(target);
                false
            }
            Some(1) => {
                // Moved one step in direction
                true
            }
            Some(2) => {
                // Moved in direction and found oxygen system
                room.set_oxygen_system(target);
                distance_to_oxygen_system = journey_back.len();
                true
            }
            _ => panic!("Unexpected program output"),
        };

        if moved {
            room.move_droid(target);
            if !backtracking {
                journey_back.push(match direction {
                    North => South,
                    South => North,
                    East => West,
                    West => East,
                });
            }
        }

        if let Some(d) = room.direction_of_unknown_space(room.droid.unwrap()) {
            direction = d;
            backtracking = false;
        } else if let Some(d) = journey_back.pop() {
            direction = d;
            backtracking = true;
        } else {
            // Nowhere left to go
            break;
        }

        thread::sleep(time::Duration::from_millis(2));
        println!("\n{}", room);
    }

    room.droid = None;
    room.start_filling_room_with_oxygen();

    let mut minutes = 0;
    while room.empty_space_count() > 0 {
        room.tick_a_minute();
        minutes += 1;
        thread::sleep(time::Duration::from_millis(2));
        println!("\n{}", room);
    }

    println!(
        "Distance from start to oxygen system: {}",
        distance_to_oxygen_system
    );

    println!("Minutes to fill room with oxygen: {}", minutes);

    show_cursor();
}
