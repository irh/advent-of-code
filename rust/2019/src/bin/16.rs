fn get_digits(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

struct PatternIterator {
    index: u32,
    order: u32,
}

impl PatternIterator {
    fn new(start: u32, order: u32) -> Self {
        assert!(order > 0);
        Self {
            index: start,
            order,
        }
    }
}

impl Iterator for PatternIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        Some(match (self.index / self.order) % 4 {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => panic!(),
        })
    }
}

fn perform_fft(signal: &mut [i32]) {
    let n = signal.len() as u32;
    for i in 0..n {
        let sum = signal
            .iter()
            .zip(PatternIterator::new(0, i + 1))
            .map(|(a, b)| a * b)
            .sum::<i32>();
        signal[i as usize] = sum.abs() % 10;
    }
}

fn main() {
    let input = get_digits(include_str!("input/16"));

    {
        // Part one
        let mut signal = input.clone();

        for _ in 0..100 {
            perform_fft(signal.as_mut_slice());
        }

        println!(
            "First 8 digits of signal after 100 rounds of Flawed Frequency Transmission: {}",
            signal[..8]
                .iter()
                .fold(0, |result, digit| result * 10 + digit)
        );
    }

    {
        // Part two
        let offset = input[..7]
            .iter()
            .fold(0, |result, digit| result * 10 + digit) as usize;

        let n = input.len() * 10_000;

        // Because the offset is past halfway through the signal, we can avoid a complete
        // calculation due to the multiplication pattern being entirely 1s past the offset.
        assert!(offset > n / 2);

        let mut signal: Vec<i32> = input.into_iter().cycle().take(n).skip(offset).collect();

        for _ in 0..100 {
            let mut sum = signal.iter().sum::<i32>();
            for x in signal.iter_mut() {
                let temp = sum;
                sum -= *x;
                *x = temp % 10;
            }
        }

        print!(
            "8-digit message in signal at offset {}: {}",
            offset,
            signal[..8]
                .iter()
                .fold(0, |result, digit| result * 10 + digit)
        );
    }
}

#[cfg(test)]
mod day_16 {
    use super::*;

    #[test]
    fn test_pattern_iterator() {
        let i = PatternIterator::new(0, 1);
        let x: Vec<i32> = i.take(8).collect();
        assert_eq!(x, vec![1, 0, -1, 0, 1, 0, -1, 0]);

        let i = PatternIterator::new(0, 2);
        let x: Vec<i32> = i.take(8).collect();
        assert_eq!(x, vec![0, 1, 1, 0, 0, -1, -1, 0]);

        let i = PatternIterator::new(0, 8);
        let x: Vec<i32> = i.take(8).collect();
        assert_eq!(x, vec![0, 0, 0, 0, 0, 0, 0, 1]);

        let i = PatternIterator::new(1, 1);
        let x: Vec<i32> = i.take(8).collect();
        assert_eq!(x, vec![0, -1, 0, 1, 0, -1, 0, 1]);

        let i = PatternIterator::new(7, 8);
        let x: Vec<i32> = i.take(8).collect();
        assert_eq!(x, vec![1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_0() {
        let mut signal = get_digits("12345678");
        assert_eq!(signal.as_slice(), [1, 2, 3, 4, 5, 6, 7, 8]);
        perform_fft(signal.as_mut_slice());
        assert_eq!(signal.as_slice(), [4, 8, 2, 2, 6, 1, 5, 8]);
        perform_fft(signal.as_mut_slice());
        assert_eq!(signal.as_slice(), [3, 4, 0, 4, 0, 4, 3, 8]);
        perform_fft(signal.as_mut_slice());
        assert_eq!(signal.as_slice(), [0, 3, 4, 1, 5, 5, 1, 8]);
        perform_fft(signal.as_mut_slice());
        assert_eq!(signal.as_slice(), [0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_1() {
        let mut signal = get_digits("80871224585914546619083218645595");
        for _ in 0..100 {
            perform_fft(signal.as_mut_slice());
        }
        assert_eq!(signal[0..8], [2, 4, 1, 7, 6, 1, 7, 6]);
    }
}
