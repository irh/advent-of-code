use std::collections::HashMap;

struct Ingredient<'a> {
    chemical: &'a str,
    quantity: u64,
}

impl<'a> Ingredient<'a> {
    pub fn new(input: &'a str) -> Self {
        let input = input.trim();
        let (quantity, chemical) = input.split_at(input.find(' ').unwrap());
        Self {
            chemical: chemical.trim(),
            quantity: quantity
                .trim()
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("Unable to parse ingredient quantity: '{input}'")),
        }
    }
}

struct ReactionMap<'a>(HashMap<&'a str, (u64, Vec<Ingredient<'a>>)>);

impl<'a> ReactionMap<'a> {
    fn new(input: &'a str) -> Self {
        let mut reactions = HashMap::new();

        for line in input.trim().lines() {
            let mut ingredients_result = line.splitn(2, "=>");
            let ingredients = ingredients_result
                .next()
                .unwrap()
                .split(',')
                .map(Ingredient::new)
                .collect();
            let result = Ingredient::new(ingredients_result.next().unwrap());
            reactions.insert(result.chemical, (result.quantity, ingredients));
        }

        reactions.insert("ORE", (1, vec![]));

        Self(reactions)
    }

    fn ore_required_for_fuel(&self, n: u64) -> u64 {
        let (quantity, inputs) = &self.0["FUEL"];
        assert_eq!(*quantity, 1);
        let mut chemicals_required = HashMap::new();
        for ingredient in inputs.iter() {
            self.get_chemical_requirements(
                ingredient.chemical,
                ingredient.quantity * n,
                &mut chemicals_required,
            );
        }
        chemicals_required["ORE"].0
    }

    fn get_chemical_requirements(
        &self,
        chemical: &'a str,
        quantity_required: u64,
        requirements: &mut HashMap<&'a str, (u64, u64)>, // total / surplus
    ) {
        let (manufacture_quantity, ingredients) = &self.0[chemical];

        let requirement = requirements.entry(chemical).or_default();
        if requirement.1 >= quantity_required {
            requirement.1 -= quantity_required;
            return;
        }

        let extra_needed = quantity_required - requirement.1;

        let mut manufactured = 0;
        let mut order = 0;
        while manufactured < extra_needed {
            manufactured += manufacture_quantity;
            order += 1;
        }

        let new_surplus = manufactured - extra_needed;

        //         println!(
        //             "Manufacturing {} - manufacture_quantity: {} required: {} extra needed: {} manufactured: {} old surplus: {} new surplus: {}",
        //             chemical,  manufacture_quantity, quantity_required, extra_needed, manufactured,requirement.1,new_surplus
        //         );

        requirement.0 += manufactured;
        requirement.1 = new_surplus;

        if manufactured > 0 {
            for ingredient in ingredients.iter() {
                self.get_chemical_requirements(
                    ingredient.chemical,
                    ingredient.quantity * order,
                    requirements,
                );
            }
        }
    }
}

fn main() {
    let reactions = ReactionMap::new(include_str!("input/14"));
    println!(
        "{} ore required for 1 fuel",
        reactions.ore_required_for_fuel(1)
    );

    let mut min = 1_000_000u64;
    let mut max = 2_000_000u64;
    while min < max {
        let mid = (min + max) / 2;
        let ore = reactions.ore_required_for_fuel(mid);
        println!("Ore required for {} fuel: {}", mid, ore);
        use std::cmp::Ordering;
        match ore.cmp(&1_000_000_000_000) {
            Ordering::Less => min = mid + 1,
            Ordering::Greater => max = mid - 1,
            Ordering::Equal => {}
        }
    }
}

#[cfg(test)]
mod day_14 {
    use super::*;

    #[test]
    fn test_0() {
        let input = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
            ";
        let reactions = ReactionMap::new(input);
        assert_eq!(31, reactions.ore_required_for_fuel(1));
    }

    #[test]
    fn test_1() {
        let input = "
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
            ";
        let reactions = ReactionMap::new(input);
        assert_eq!(165, reactions.ore_required_for_fuel(1));
    }

    #[test]
    fn test_2() {
        let input = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
            ";
        let reactions = ReactionMap::new(input);
        assert_eq!(13312, reactions.ore_required_for_fuel(1));
    }

    #[test]
    fn test_3() {
        let input = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
            ";
        let reactions = ReactionMap::new(input);
        assert_eq!(180697, reactions.ore_required_for_fuel(1));
    }

    #[test]
    fn test_4() {
        let input = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
            ";
        let reactions = ReactionMap::new(input);
        assert_eq!(2210736, reactions.ore_required_for_fuel(1));
    }
}
