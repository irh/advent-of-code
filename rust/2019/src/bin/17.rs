mod intcode;

use {
    intcode::Program,
    std::{
        collections::{HashMap, HashSet},
        fmt::{self, Debug, Display},
    },
};

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Object {
    Empty,
    Scaffold,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn location_in_direction(v: Vec2, d: Direction) -> Vec2 {
    use Direction::*;
    match d {
        North => Vec2 { x: v.x, y: v.y - 1 },
        East => Vec2 { x: v.x + 1, y: v.y },
        South => Vec2 { x: v.x, y: v.y + 1 },
        West => Vec2 { x: v.x - 1, y: v.y },
    }
}

#[derive(Copy, Clone)]
struct Robot {
    location: Vec2,
    direction: Direction,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Command {
    Left(u32),
    Right(u32),
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Command::Left(x) => write!(f, "L,{}", x),
            Command::Right(x) => write!(f, "R,{}", x),
        }
    }
}

struct Room {
    grid: HashMap<Vec2, Object>,
    intersections: HashSet<Vec2>,
    robot: Robot,
}

impl Room {
    pub fn new() -> Self {
        Self {
            grid: Default::default(),
            intersections: Default::default(),
            robot: Robot {
                location: Default::default(),
                direction: Direction::North,
            },
        }
    }

    fn bounds(&self) -> Vec2 {
        let mut x = 0;
        let mut y = 0;
        for l in self.grid.keys() {
            assert!(l.x >= 0);
            assert!(l.y >= 0);
            x = x.max(l.x);
            y = y.max(l.y);
        }
        Vec2 { x, y }
    }

    fn find_intersections(&mut self) {
        use Object::*;

        let bounds = self.bounds();
        for x in 1..=(bounds.x - 1) {
            for y in 1..=(bounds.y - 1) {
                if self.grid[&Vec2 { x, y }] == Scaffold
                    && self.grid[&Vec2 { x: x - 1, y }] == Scaffold
                    && self.grid[&Vec2 { x: x + 1, y }] == Scaffold
                    && self.grid[&Vec2 { x, y: y - 1 }] == Scaffold
                    && self.grid[&Vec2 { x, y: y + 1 }] == Scaffold
                {
                    self.intersections.insert(Vec2 { x, y });
                }
            }
        }
    }

    fn find_robot_route(&self) -> Vec<Command> {
        use {Command::*, Direction::*, Object::*};

        let mut result = Vec::new();
        let mut visited = HashSet::new();

        let mut robot = self.robot;

        visited.insert(robot.location);
        loop {
            let new_direction = (|| {
                let location = Vec2 {
                    x: robot.location.x - 1,
                    y: robot.location.y,
                };
                if self.grid.get(&location) == Some(&Scaffold) && !visited.contains(&location) {
                    return Some(West);
                }
                let location = Vec2 {
                    x: robot.location.x + 1,
                    y: robot.location.y,
                };
                if self.grid.get(&location) == Some(&Scaffold) && !visited.contains(&location) {
                    return Some(East);
                }
                let location = Vec2 {
                    x: robot.location.x,
                    y: robot.location.y - 1,
                };
                if self.grid.get(&location) == Some(&Scaffold) && !visited.contains(&location) {
                    return Some(North);
                }
                let location = Vec2 {
                    x: robot.location.x,
                    y: robot.location.y + 1,
                };
                if self.grid.get(&location) == Some(&Scaffold) && !visited.contains(&location) {
                    return Some(South);
                }
                None
            })();

            if new_direction.is_none() {
                break;
            }

            let new_direction = new_direction.unwrap();

            let mut steps = 0;
            loop {
                let target = location_in_direction(robot.location, new_direction);
                if self.grid.get(&target) == Some(&Scaffold) {
                    robot.location = target;
                    visited.insert(target);
                    steps += 1;
                } else {
                    break;
                }
            }

            result.push(match (robot.direction, new_direction) {
                (North, East) | (East, South) | (South, West) | (West, North) => Right(steps),
                (North, West) | (West, South) | (South, East) | (East, North) => Left(steps),
                _ => panic!(
                    "Something went wrong while deciding where to turn \
                     - robot.direction: {:?} new_direction: {:?}",
                    robot.direction, new_direction
                ),
            });

            robot.direction = new_direction;
        }

        result
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use {Direction::*, Object::*};

        let bounds = self.bounds();

        for y in 0..=bounds.y {
            for x in 0..=bounds.x {
                let location = Vec2 { x, y };
                if location == self.robot.location {
                    match self.robot.direction {
                        North => write!(f, "^")?,
                        East => write!(f, ">")?,
                        South => write!(f, "v")?,
                        West => write!(f, "<")?,
                    };
                } else {
                    match self.grid[&location] {
                        Empty => write!(f, ".")?,
                        Scaffold => {
                            if self.intersections.contains(&location) {
                                write!(f, "O")?;
                            } else {
                                write!(f, "#")?;
                            }
                        }
                    };
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct MovementProgram {
    main: Vec<String>,
    a: Vec<Command>,
    b: Vec<Command>,
    c: Vec<Command>,
}

impl MovementProgram {
    fn compress_route(route: &Vec<Command>) -> Self {
        // We want to split the route into 3 movement functions;
        // each function can have between 1 and 5 commands
        let function_min = 1;
        let function_max = 5;
        assert!(route.len() >= function_max * 3);

        for len_a in function_min..=function_max {
            // The A function always starts at the beginning of the route
            let a = &route[0..len_a];

            for offset_b in len_a..(route.len() - function_max * 2) {
                for len_b in function_min..=function_max {
                    // The B function can start anywhere after the A function, while leaving room
                    // for the C function
                    let b = &route[offset_b..offset_b + len_b];

                    if a == b {
                        break;
                    }

                    for offset_c in (offset_b + len_b)..(route.len() - function_max) {
                        'try_next: for len_c in function_min..=function_max {
                            // The C function can start anywhere after the B function
                            let c = &route[offset_c..offset_c + len_c];

                            if a == c || b == c {
                                break;
                            }

                            let mut main = Vec::new();
                            let mut i = 0;
                            while i < route.len() {
                                if a.len() <= (route.len() - i) && a == &route[i..i + a.len()] {
                                    main.push("A".to_string());
                                    i += a.len();
                                } else if b.len() <= (route.len() - i)
                                    && b == &route[i..i + b.len()]
                                {
                                    main.push("B".to_string());
                                    i += b.len();
                                } else if c.len() <= (route.len() - i)
                                    && c == &route[i..i + c.len()]
                                {
                                    main.push("C".to_string());
                                    i += c.len();
                                } else {
                                    // If we didn't find a match for the current position, then try
                                    // the next compression
                                    continue 'try_next;
                                }
                            }

                            // If we've matched sections of the whole program then we have our answer
                            return Self {
                                main,
                                a: a.to_vec(),
                                b: b.to_vec(),
                                c: c.to_vec(),
                            };
                        }
                    }
                }
            }
        }

        Self {
            ..Default::default()
        }
    }
}

impl Display for MovementProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_function = |function: &Vec<Command>| {
            function
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join(",")
        };

        write!(
            f,
            "{}\n{}\n{}\n{}\n",
            self.main.join(","),
            fmt_function(&self.a),
            fmt_function(&self.b),
            fmt_function(&self.c)
        )?;

        Ok(())
    }
}

fn string_to_intcode(s: &str) -> Vec<intcode::Value> {
    s.chars()
        .map(|c| c as u8 as intcode::Value)
        .collect::<Vec<intcode::Value>>()
}

fn main() {
    use {Direction::*, Object::*};

    let mut room = Room::new();

    {
        let mut x = 0;
        let mut y = 0;

        for a in Program::new(include_str!("input/17")) {
            let a = a as u8 as char;
            let location = Vec2 { x, y };
            x += 1;
            match a {
                '#' => {
                    room.grid.insert(location, Scaffold);
                }
                '.' => {
                    room.grid.insert(location, Empty);
                }
                '^' => {
                    room.robot = Robot {
                        location,
                        direction: North,
                    };
                    room.grid.insert(location, Scaffold);
                }
                '>' => {
                    room.robot = Robot {
                        location,
                        direction: East,
                    };
                    room.grid.insert(location, Scaffold);
                }
                'v' => {
                    room.robot = Robot {
                        location,
                        direction: South,
                    };
                    room.grid.insert(location, Scaffold);
                }
                '<' => {
                    room.robot = Robot {
                        location,
                        direction: West,
                    };
                    room.grid.insert(location, Scaffold);
                }
                '\n' => {
                    y += 1;
                    x = 0;
                }
                _ => panic!("Unexpected program output"),
            }
        }

        room.find_intersections();

        println!("{}", room);

        println!(
            "Sum of intersection alignment parameters: {}",
            room.intersections.iter().map(|v| v.x * v.y).sum::<i32>()
        );
    }

    {
        let route = room.find_robot_route();
        let compressed_route = MovementProgram::compress_route(&route);

        let mut program = Program::new(include_str!("input/17"));
        program.write(0, 2);

        let serialized = format!("{}", compressed_route);
        println!("\nMovement commands:\n{}", serialized);

        let input = string_to_intcode(&(serialized + "n\n"));
        program.set_input(&input);

        let mut result = 0;
        for x in program {
            result = x;
        }

        println!("Dust collected: {}", result);
    }
}
