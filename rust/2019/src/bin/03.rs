// https://adventofcode.com/2019/day/3

use std::collections::HashSet;

type Point = (i32, i32);
type Wire = Vec<Point>;

fn make_wire(commands: &str) -> Wire {
    let mut wire = Wire::new();
    let mut x = 0;
    let mut y = 0;
    for command in commands.split(',') {
        let (direction, distance) = command.split_at(1);
        let distance = distance.parse::<i32>().expect("Unable to parse distance");
        match direction {
            "L" => {
                for step in 1..distance + 1 {
                    wire.push((x - step, y));
                }
                x -= distance;
            }
            "R" => {
                for step in 1..distance + 1 {
                    wire.push((x + step, y));
                }
                x += distance;
            }
            "D" => {
                for step in 1..distance + 1 {
                    wire.push((x, y - step));
                }
                y -= distance;
            }
            "U" => {
                for step in 1..distance + 1 {
                    wire.push((x, y + step));
                }
                y += distance;
            }
            _ => panic!("Unexpected direction"),
        }
    }
    wire
}

fn manhattan(p: &Point) -> u32 {
    (p.0.abs() + p.1.abs()) as u32
}

fn closest_intersection_by_distance(wire_a: &Wire, wire_b: &Wire) -> u32 {
    let a_hash: HashSet<Point> = wire_a.iter().cloned().collect();
    let b_hash: HashSet<Point> = wire_b.iter().cloned().collect();
    manhattan(
        a_hash
            .intersection(&b_hash)
            .min_by(|a, b| manhattan(a).cmp(&manhattan(b)))
            .expect("No intersections found"),
    )
}

fn closest_intersection_by_signal_delay(a: &Wire, b: &Wire) -> usize {
    let a_hash: HashSet<Point> = a.iter().cloned().collect();
    let b_hash: HashSet<Point> = b.iter().cloned().collect();
    a_hash
        .intersection(&b_hash)
        .map(|p| {
            (a.iter().position(|x| p == x).unwrap() + 1)
                + (b.iter().position(|x| p == x).unwrap() + 1)
        })
        .min()
        .expect("No intersections found")
}

fn main() {
    let input = include_str!("input/3");
    let a = make_wire(input.lines().next().unwrap());
    let b = make_wire(input.lines().nth(1).unwrap());
    println!(
        "Closest intersection by manhattan distance: {}",
        closest_intersection_by_distance(&a, &b)
    );
    println!(
        "Closest intersection by signal delay: {}",
        closest_intersection_by_signal_delay(&a, &b)
    );
}

#[cfg(test)]
mod day_3 {
    use super::*;

    #[test]
    fn test_0() {
        let a = make_wire("R8,U5,L5,D3");
        let b = make_wire("U7,R6,D4,L4");
        assert_eq!(6, closest_intersection_by_distance(&a, &b));
        assert_eq!(30, closest_intersection_by_signal_delay(&a, &b));
    }

    #[test]
    fn test_1() {
        let a = make_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let b = make_wire("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(159, closest_intersection_by_distance(&a, &b));
        assert_eq!(610, closest_intersection_by_signal_delay(&a, &b));
    }

    #[test]
    fn test_2() {
        let a = make_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let b = make_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(135, closest_intersection_by_distance(&a, &b));
        assert_eq!(410, closest_intersection_by_signal_delay(&a, &b));
    }
}
