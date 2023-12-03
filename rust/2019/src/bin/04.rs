// https://adventofcode.com/2019/day/4

fn main() {
    let min = 136818;
    let max = 685979;

    let password_count = (min..max + 1)
        .filter(|x| is_valid_password_part_one(&get_digits(*x)))
        .count();
    println!("Number of valid passwords - part one: {}", password_count);

    let password_count = (min..max + 1)
        .filter(|x| is_valid_password_part_two(&get_digits(*x)))
        .count();
    println!("Number of valid passwords - part two: {}", password_count);
}

fn get_digits(number: u32) -> Vec<u8> {
    if number < 10 {
        return vec![number as u8];
    }

    let count = (number as f64).log10().floor() as usize + 1;
    let mut digits = Vec::with_capacity(count);
    let mut x = number;
    while x > 0 {
        digits.insert(0, (x % 10) as u8);
        x /= 10;
    }

    digits
}

fn is_valid_password_part_one(digits: &[u8]) -> bool {
    let mut double_found = false;
    let mut x = 0;
    for d in digits.iter() {
        match *d {
            d if d < x => return false,
            d if d == x => double_found = true,
            _ => (),
        }
        x = *d;
    }
    double_found
}

fn is_valid_password_part_two(digits: &[u8]) -> bool {
    let mut double_found = false;
    let mut group_count = 1;
    let mut current = 0;

    for d in digits.iter() {
        match *d {
            d if d < current => return false,
            d if d == current => {
                group_count += 1;
            }
            _ => {
                if group_count == 2 {
                    double_found = true;
                }
                group_count = 1;
            }
        }
        current = *d;
    }

    if group_count == 2 {
        double_found = true;
    }

    double_found
}

#[cfg(test)]
mod day_4 {
    use super::*;

    #[test]
    fn test_get_digits() {
        assert_eq!(vec![0], get_digits(0));
        assert_eq!(vec![9], get_digits(9));
        assert_eq!(vec![4, 2], get_digits(42));
        assert_eq!(vec![1, 2, 3], get_digits(123));
        assert_eq!(vec![5, 4, 1, 0], get_digits(5410));
        assert_eq!(vec![8, 9, 1, 7, 4], get_digits(89174));
        assert_eq!(vec![2, 5, 0, 8, 6, 1], get_digits(250861));
    }

    #[test]
    fn test_is_valid_password_part_one() {
        assert!(is_valid_password_part_one(&vec![1, 1, 1, 1, 1, 1]));
        assert!(is_valid_password_part_one(&vec![1, 1, 1, 1, 2, 2]));
        assert!(is_valid_password_part_one(&vec![1, 1, 1, 1, 2, 3]));
        assert!(is_valid_password_part_one(&vec![1, 2, 2, 3, 4, 5]));
        assert!(!is_valid_password_part_one(&vec![2, 2, 3, 4, 5, 0]));
        assert!(!is_valid_password_part_one(&vec![1, 2, 3, 7, 8, 9]));
    }

    #[test]
    fn test_is_valid_password_part_two() {
        assert!(!is_valid_password_part_two(&vec![1, 1, 1, 1, 1, 1]));
        assert!(is_valid_password_part_two(&vec![1, 1, 2, 2, 3, 3]));
        assert!(is_valid_password_part_two(&vec![1, 1, 1, 1, 2, 2]));
        assert!(!is_valid_password_part_two(&vec![1, 1, 1, 1, 2, 3]));
        assert!(!is_valid_password_part_two(&vec![1, 2, 3, 4, 4, 4]));
        assert!(is_valid_password_part_two(&vec![1, 2, 2, 3, 4, 5]));
        assert!(!is_valid_password_part_two(&vec![2, 2, 3, 4, 5, 0]));
        assert!(!is_valid_password_part_two(&vec![1, 2, 3, 7, 8, 9]));
    }
}
