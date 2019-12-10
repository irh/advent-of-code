// https://adventofcode.com/2019/day/10

use ordered_float::OrderedFloat;
use std::{collections::HashSet, iter::FromIterator};

fn gcd(a: isize, b: isize) -> isize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let x = a % b;
        a = b;
        b = x;
    }
    a
}

type Location = (usize, usize);
type Direction = (isize, isize);

#[derive(Clone)]
enum Object {
    Empty,
    Asteroid,
}

#[derive(Clone)]
struct Map(Vec<Vec<Object>>);

impl Map {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<Object>> = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|x| match x {
                        '.' => Some(Object::Empty),
                        '#' => Some(Object::Asteroid),
                        _ => panic!(format!("Unexpected location type: {}", x)),
                    })
                    .collect()
            })
            .collect();

        Self(grid)
    }

    fn best_monitoring_station_location(&self) -> Option<(Location, usize)> {
        use Object::*;

        let mut best_location = None;
        let mut best_count = 0;

        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Asteroid => {
                        let directions = self.asteroid_directions((x, y));

                        if directions.len() > best_count {
                            best_location = Some((x, y));
                            best_count = directions.len();
                        }
                    }
                    Empty => (),
                }
            }
        }

        if let Some(location) = best_location {
            Some((location, best_count))
        } else {
            None
        }
    }

    fn asteroid_directions(&self, (x, y): Location) -> HashSet<Direction> {
        let mut directions = HashSet::new();

        for (y2, row) in self.0.iter().enumerate() {
            for (x2, cell) in row.iter().enumerate() {
                match cell {
                    Object::Asteroid if !(x == x2 && y == y2) => {
                        let dx = x2 as isize - x as isize;
                        let dy = y2 as isize - y as isize;
                        let d = gcd(dx, dy).abs();
                        directions.insert((dx / d, dy / d));
                    }
                    _ => (),
                }
            }
        }

        directions
    }
}

struct Vaporizer {
    map: Map,
    laser: Location,
    directions: Vec<Direction>,
    direction: usize,
}

impl Vaporizer {
    pub fn new(map: &Map, laser: Location) -> Self {
        let mut directions: Vec<Direction> =
            Vec::from_iter(map.asteroid_directions(laser).into_iter());
        // Sort the directions so that we start vertically and then rotate clockwise
        directions.sort_by_cached_key(|d| OrderedFloat(-(d.0 as f64).atan2(d.1 as f64)));
        Self {
            map: map.clone(),
            laser,
            directions,
            direction: 0,
        }
    }
}

impl Iterator for Vaporizer {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        let width = self.map.0.first().unwrap().len();
        let height = self.map.0.len();

        while !self.directions.is_empty() {
            let (dx, dy) = self.directions[self.direction];

            let mut x = self.laser.0 as isize;
            let mut y = self.laser.1 as isize;

            loop {
                x += dx;
                y += dy;

                let x = x as usize;
                let y = y as usize;

                if !(0..width).contains(&x) || !(0..height).contains(&y) {
                    break;
                }

                if let Object::Asteroid = self.map.0[y][x] {
                    // Found an asteroid in the current direction, vaporize it and move to the next
                    // direction, returning the location of the vaporized asteroid
                    self.map.0[y][x] = Object::Empty;
                    self.direction = (self.direction + 1) % self.directions.len();
                    return Some((x, y));
                }
            }

            // There aren't any asteroids left in this direction, so remove it and try the next one
            self.directions.remove(self.direction);
            if self.direction == self.directions.len() {
                self.direction = 0;
            }
        }

        None
    }
}

fn main() {
    let map = Map::new(include_str!("input/10"));

    let (pos, count) = map.best_monitoring_station_location().unwrap();
    println!(
        "Best location - {} asteroids detected from location {:?}",
        count, pos
    );

    let mut vaporizer = Vaporizer::new(&map, pos);
    println!(
        "200th asteroid to be vaporized is at location {:?}",
        vaporizer.nth(199).unwrap()
    );
}

#[cfg(test)]
mod day_10 {
    use super::*;

    #[test]
    fn test_0() {
        let data = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
        let map = Map::new(data);
        assert_eq!(Some(((5, 8), 33)), map.best_monitoring_station_location());
    }

    #[test]
    fn test_1() {
        let data = "
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";
        let map = Map::new(data);
        assert_eq!(Some(((1, 2), 35)), map.best_monitoring_station_location());
    }

    #[test]
    fn test_2() {
        let data = "
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##
";
        let map = Map::new(data);
        let vaporized: Vec<Location> = Vaporizer::new(&map, (8, 3)).collect();
        assert_eq!((8, 1), vaporized[0]);
        assert_eq!((9, 0), vaporized[1]);
        assert_eq!((9, 1), vaporized[2]);
        assert_eq!((10, 0), vaporized[3]);
        assert_eq!((9, 2), vaporized[4]);
        assert_eq!((11, 1), vaporized[5]);
    }

    #[test]
    fn test_3() {
        let data = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
        let map = Map::new(data);
        let (pos, count) = map.best_monitoring_station_location().unwrap();
        assert_eq!((11, 13), pos);
        assert_eq!(210, count);
        let vaporized: Vec<Location> = Vaporizer::new(&map, pos).collect();
        assert_eq!((11, 12), vaporized[0]);
        assert_eq!((12, 1), vaporized[1]);
        assert_eq!((12, 2), vaporized[2]);
        assert_eq!((12, 8), vaporized[9]);
        assert_eq!((16, 0), vaporized[19]);
        assert_eq!((16, 9), vaporized[49]);
        assert_eq!((10, 16), vaporized[99]);
        assert_eq!((9, 6), vaporized[198]);
        assert_eq!((8, 2), vaporized[199]);
        assert_eq!((10, 9), vaporized[200]);
        assert_eq!((11, 1), vaporized[298]);
        assert_eq!(299, vaporized.len());
    }
}
