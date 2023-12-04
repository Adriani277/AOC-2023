fn main() {
    let input = include_str!("./input1.txt");
    println!("result {}", calibration_sum(input))
}

pub mod part1 {
    pub const AS_DIGIT: fn(char) -> Option<u32>  = {
        |c: char| c.to_digit(10)
    };
}

use part1::AS_DIGIT;

fn calibration_sum(input: &str) -> u32 {
    input.lines().into_iter().fold(0, |acc, line| {
        let mut digits = (0..line.len())
            .map(|i| &line[i..])
            .filter_map(|v| get_digits(v));

        let first = digits.next().unwrap();
        let last = digits.next_back().unwrap_or(first);

        let number = (first * 10) + last;

        // println!("acc: {:?}", acc + number);
        acc + number
    })
}

static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digits(line: &str) -> Option<u32> {
    line.chars()
        .next()
        .and_then(AS_DIGIT)
        .or_else(|| {
            NUMBERS
                .iter()
                .position(|n| line.starts_with(n))
                .map(|p| p as u32 + 1)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_sum() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(calibration_sum(input), 281);
    }

    #[test]
    fn test_get_digits() {
        let input = "zoneight234";

        let mut result = (0..input.len())
            .map(|i| &input[i..])
            .filter_map(|f| get_digits(f));
        print!("{:?}", result);
        assert_eq!(result.next(), Some(1));
        assert_eq!(result.next_back(), Some(4));
    }
}
