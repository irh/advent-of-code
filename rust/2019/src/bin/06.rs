// https://adventofcode.com/2019/day/6

use std::collections::HashMap;

// struct Object(Vec<Object>);
// impl Object {
//     fn new() -> Self {
//         Self(Vec::new())
//     }
// }

struct OrbitMap<'a> {
    objects: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> OrbitMap<'a> {
    fn new(map_data: &'a str) -> Self {
        let mut objects = HashMap::new();

        for orbit in map_data.lines() {
            let mut orbit_objects = orbit.splitn(2, ')');
            let parent = orbit_objects.next().expect("Failed to parse parent object");
            let child = orbit_objects
                .next()
                .unwrap_or_else(|| panic!("Failed to parse child object - '{}'", orbit));

            objects.entry(child).or_insert(Vec::new());
            objects.entry(parent).or_insert(Vec::new()).push(child);
        }

        assert!(objects.contains_key("COM"));

        Self { objects }
    }

    fn checksum(&self) -> usize {
        self.checksum_for_object("COM", 1)
    }

    fn checksum_for_object(&self, object: &str, total: usize) -> usize {
        self.objects
            .get(object)
            .unwrap_or_else(|| panic!("No object named {object}"))
            .iter()
            .fold(0, |x, child| {
                x + total + self.checksum_for_object(child, total + 1)
            })
    }

    fn transfers_from_a_to_b(&self, a: &str, b: &str) -> Option<usize> {
        let path_a = self.path_to_target("COM", a);
        let path_b = self.path_to_target("COM", b);

        if path_a.is_empty() || path_b.is_empty() {
            return None;
        }

        Some(
            // find the first object that isn't in both paths
            match path_a.iter().zip(path_b.iter()).position(|(a, b)| a != b) {
                Some(index_of_shared_root) => {
                    // the result is the length of the non-matching sections of the paths
                    path_a.len() + path_b.len() - 2 * index_of_shared_root
                }
                None => {
                    // when no mismatch was found, one of the objects is orbiting the shared root
                    if path_a.len() > path_b.len() {
                        path_a.len() - path_b.len()
                    } else {
                        path_b.len() - path_a.len()
                    }
                }
            },
        )
    }

    fn path_to_target(&self, start: &'a str, target: &str) -> Vec<&'a str> {
        let orbits = self
            .objects
            .get(start)
            .unwrap_or_else(|| panic!("path_to_target - no object named {start}"));

        for orbit in orbits.iter() {
            if *orbit == target {
                return vec![start];
            } else {
                let mut path = self.path_to_target(orbit, target);
                if !path.is_empty() {
                    path.insert(0, start);
                    return path;
                }
            }
        }

        vec![]
    }
}

fn main() {
    let input = include_str!("input/6");
    let map = OrbitMap::new(input);
    println!("Map checksum: {}", map.checksum());
    println!(
        "Transfers from you to Santa: {:?}",
        map.transfers_from_a_to_b("YOU", "SAN").unwrap()
    );
}

#[cfg(test)]
mod day_6 {
    use super::*;

    #[test]
    fn test_checksum() {
        let input = include_str!("input/6-test");
        let map = OrbitMap::new(input);
        assert_eq!(42, map.checksum());
    }

    #[test]
    fn test_transfers() {
        let input = include_str!("input/6-test-2");
        let map = OrbitMap::new(input);

        assert_eq!(Some(0), map.transfers_from_a_to_b("YOU", "L"));
        assert_eq!(Some(2), map.transfers_from_a_to_b("YOU", "F"));
        assert_eq!(Some(4), map.transfers_from_a_to_b("YOU", "SAN"));
    }
}
