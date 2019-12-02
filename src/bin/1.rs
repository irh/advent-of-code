// https://adventofcode.com/2019/day/1

fn main() {
    let input = include_str!("input/1");

    let mut result_no_extra_fuel = 0;
    let mut result_with_extra_fuel = 0;

    for line in input.lines() {
        let mass = line
            .parse::<i32>()
            .expect(&format!("Unable to parse mass: {}", line));

        let fuel = mass / 3 - 2;

        let mut extra_fuel = 0;
        let mut fuel_temp = fuel;
        loop {
            fuel_temp = fuel_temp / 3 - 2;
            if fuel_temp <= 0 {
                break;
            }
            extra_fuel += fuel_temp;
        }

        result_no_extra_fuel += fuel;
        result_with_extra_fuel += fuel + extra_fuel;
    }

    println!("Without fuel: {}", result_no_extra_fuel);
    println!("With fuel: {}", result_with_extra_fuel);
}
