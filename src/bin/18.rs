#[derive(Clone, PartialEq)]
enum Object {
    Empty,
    Wall,
    Entrance,
    Key(char),
    Door(char),
}

struct VaultTree {
    object: Object,
    children: Vec<VaultTree>,
}

struct Vault {
    tree: VaultTree,
}

impl Vault {
    pub fn new(input: &str) -> Self {
        use Object::*;

        let objects = input
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| match c {
                        '.' => Empty,
                        '#' => Wall,
                        '@' => Entrance,
                        c @ 'a'..='z' => Key(c),
                        c @ 'A'..='Z' => Door(c),
                        _ => panic!("Unexpected input"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .concat();

        let entrance = objects.iter().position(|o| *o == Entrance);

        Self {
            tree: VaultTree {
                object: Entrance,
                children: vec![],
            },
        }
    }
}

fn main() {}

#[cfg(test)]
mod day_18 {
    use super::*;

    #[test]
    fn test_0() {
        let input = "
#########
#b.A.@.a#
#########
";
    }

    // #[test]
    // fn test_1() {
    //     let input = "
    // ########################
    // #f.D.E.e.C.b.A.@.a.B.c.#
    // ######################.#
    // #d.....................#
    // ########################
    // ";
    // }

    // #[test]
    // fn test_2() {
    //     let input = "
    // ########################
    // #...............b.C.D.f#
    // #.######################
    // #.....@.a.B.c.d.A.e.F.g#
    // ########################
    // ";
    // }
}
