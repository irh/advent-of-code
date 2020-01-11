use std::collections::HashSet;

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn position(x: usize, y: usize) -> Position {
    Position { x, y }
}

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
            .filter(|row| !row.is_empty())
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
            tree: Vault::make_tree_from(
                position(entrance_x, entrance_y),
                &grid,
                &mut HashSet::new(),
            ),
        }
    }

    fn make_tree_from(
        p: Position,
        grid: &Vec<Vec<Object>>,
        visited: &mut HashSet<Position>,
    ) -> VaultTree {
        visited.insert(p);

        let mut children = Vec::new();
        let mut make_child_tree = |child: Position| {
            if grid[child.y][child.x] != Object::Wall && !visited.contains(&child) {
                children.push(Vault::make_tree_from(child, &grid, visited));
            }
        };

        if p.x > 0 {
            make_child_tree(position(p.x - 1, p.y));
        }
        if p.x < grid.first().unwrap().len() - 1 {
            make_child_tree(position(p.x + 1, p.y));
        }
        if p.y > 0 {
            make_child_tree(position(p.x, p.y - 1));
        }
        if p.y < grid.len() - 1 {
            make_child_tree(position(p.x, p.y + 1));
        }

        VaultTree {
            object: grid[p.y][p.x],
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
        assert!(vault.tree.children.len() == 2);
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
