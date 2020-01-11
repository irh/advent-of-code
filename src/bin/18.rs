use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
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
    grid: Vec<Vec<Object>>,
    tree: VaultTree,
}

impl Vault {
    pub fn new(input: &str) -> Self {
        use Object::*;

        let grid = input
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
            .collect::<Vec<_>>();

        let entrance_y = grid
            .iter()
            .position(|row| row.iter().any(|o| *o == Entrance))
            .unwrap();
        let entrance_x = grid[entrance_y]
            .iter()
            .position(|o| *o == Entrance)
            .unwrap();

        Self {
            grid: grid.clone(),
            tree: Vault::make_tree_from((entrance_x, entrance_y), &grid, &mut HashSet::new()),
        }
    }

    fn make_tree_from(
        pos: (usize, usize),
        grid: &Vec<Vec<Object>>,
        visited: &mut HashSet<(usize, usize)>,
    ) -> VaultTree {
        visited.insert(pos);

        let mut children = Vec::new();
        let mut make_child_tree = |child: (usize, usize)| {
            if grid[child.0][child.1] != Object::Wall && !visited.contains(&child) {
                children.push(Vault::make_tree_from(child, &grid, visited));
            }
        };

        if pos.0 > 0 {
            make_child_tree((pos.0 - 1, pos.1));
        }
        if pos.0 < grid.first().unwrap().len() - 1 {
            make_child_tree((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            make_child_tree((pos.0, pos.1 - 1));
        }
        if pos.1 < grid.len() - 1 {
            make_child_tree((pos.0, pos.1 + 1));
        }

        VaultTree {
            object: grid[pos.0][pos.1],
            children,
        }
    }
}

fn main() {
    let input = include_str!("input/18");
    let _vault = Vault::new(input);
}

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
        let vault = Vault::new(input);
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
