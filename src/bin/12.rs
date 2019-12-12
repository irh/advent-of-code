// https://adventofcode.com/2019/day/12

use std::ops::AddAssign;

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let x = a % b;
        a = b;
        b = x;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Clone, Copy)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(input: &str) -> Self {
        let r = regex::Regex::new(r"(-?[0-9]+)").unwrap();
        let mut numbers = r
            .find_iter(input)
            .map(|n| n.as_str().parse::<i32>().unwrap());
        let x = numbers.next().unwrap();
        let y = numbers.next().unwrap();
        let z = numbers.next().unwrap();
        Moon {
            pos: Vec3 { x, y, z },
            vel: Vec3::default(),
        }
    }

    fn adjust_position(&mut self) {
        self.pos += self.vel;
    }

    fn adjust_velocity(&mut self, other: Self) {
        if self.pos.x != other.pos.x {
            if self.pos.x < other.pos.x {
                self.vel.x += 1;
            } else {
                self.vel.x -= 1;
            }
        }
        if self.pos.y != other.pos.y {
            if self.pos.y < other.pos.y {
                self.vel.y += 1;
            } else {
                self.vel.y -= 1;
            }
        }
        if self.pos.z != other.pos.z {
            if self.pos.z < other.pos.z {
                self.vel.z += 1;
            } else {
                self.vel.z -= 1;
            }
        }
    }

    fn energy(&self) -> i32 {
        self.pos.energy() * self.vel.energy()
    }
}

struct System {
    moons: Vec<Moon>,
}

impl System {
    fn new(input: &str) -> Self {
        System {
            moons: input.trim().lines().map(|line| Moon::new(line)).collect(),
        }
    }

    fn step(&mut self) {
        for a in 0..self.moons.len() {
            for b in a..self.moons.len() {
                let moon_a = self.moons[a];
                let moon_b = self.moons[b];
                self.moons[a].adjust_velocity(moon_b);
                self.moons[b].adjust_velocity(moon_a);
            }
        }
        for moon in self.moons.iter_mut() {
            moon.adjust_position();
        }
    }

    fn step_n(&mut self, n: u32) {
        for _ in 0..n {
            self.step();
        }
    }

    fn energy(&self) -> i32 {
        self.moons.iter().fold(0, |e, moon| e + moon.energy())
    }

    fn xs(&self) -> Vec<(i32, i32)> {
        self.moons
            .iter()
            .map(|moon| (moon.pos.x, moon.vel.x))
            .collect()
    }

    fn ys(&self) -> Vec<(i32, i32)> {
        self.moons
            .iter()
            .map(|moon| (moon.pos.y, moon.vel.y))
            .collect()
    }

    fn zs(&self) -> Vec<(i32, i32)> {
        self.moons
            .iter()
            .map(|moon| (moon.pos.z, moon.vel.z))
            .collect()
    }

    fn cycle_length(&mut self) -> usize {
        let initial_xs = self.xs();
        let initial_ys = self.ys();
        let initial_zs = self.zs();

        let mut steps_x = None;
        let mut steps_y = None;
        let mut steps_z = None;
        let mut count = 0;

        while steps_x.is_none() || steps_y.is_none() || steps_z.is_none() {
            self.step();
            count += 1;
            if steps_x.is_none() && self.xs() == initial_xs {
                steps_x = Some(count);
            }
            if steps_y.is_none() && self.ys() == initial_ys {
                steps_y = Some(count);
            }
            if steps_z.is_none() && self.zs() == initial_zs {
                steps_z = Some(count);
            }
        }

        lcm(steps_x.unwrap(), lcm(steps_y.unwrap(), steps_z.unwrap()))
    }
}

fn main() {
    let input = "
<x=-4, y=-14, z=8>
<x=1, y=-8, z=10>
<x=-15, y=2, z=1>
<x=-17, y=-17, z=16>
";
    {
        let mut system = System::new(input);
        system.step_n(1000);
        println!(
            "Total energy of system after 1000 steps: {}",
            system.energy()
        );
    }

    {
        let mut system = System::new(input);
        println!(
            "Steps before reaching a previous state: {}",
            system.cycle_length()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec3(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[test]
    fn test_0() {
        let input = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";
        let mut system = System::new(input);
        assert_eq!(4, system.moons.len());
        assert_eq!(vec3(-1, 0, 2), system.moons[0].pos);
        assert_eq!(vec3(3, 5, -1), system.moons[3].pos);

        system.step();
        assert_eq!(vec3(2, -1, 1), system.moons[0].pos);
        assert_eq!(vec3(3, -1, -1), system.moons[0].vel);
        assert_eq!(vec3(2, 2, 0), system.moons[3].pos);
        assert_eq!(vec3(-1, -3, 1), system.moons[3].vel);

        system.step_n(9);
        assert_eq!(vec3(1, -8, 0), system.moons[1].pos);
        assert_eq!(vec3(-1, 1, 3), system.moons[1].vel);
        assert_eq!(vec3(3, -6, 1), system.moons[2].pos);
        assert_eq!(vec3(3, 2, -3), system.moons[2].vel);
        assert_eq!(179, system.energy());

        assert_eq!(2772, System::new(input).cycle_length());
    }

    #[test]
    fn test_1() {
        let input = "
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
";
        assert_eq!(4686774924, System::new(input).cycle_length());
    }
}
