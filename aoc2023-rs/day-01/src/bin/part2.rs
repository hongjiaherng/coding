#[macro_use]
extern crate lazy_static;
use fancy_regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref SPELLED_DIGIT_2_DIGIT: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m
    };
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine)).*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    // iterate over each line
    return input
        .lines()
        .map(|line| {
            // use regex to find the first and last spelled digits or actual digits
            let cap = re.captures(line).unwrap().unwrap();
            let first = cap.get(1).unwrap().as_str();
            let last = cap.get(2).unwrap().as_str();

            // convert spelled digits to actual digits
            let first_digit = match first.parse::<i32>() {
                Ok(num) => num,
                Err(_) => *SPELLED_DIGIT_2_DIGIT.get(first).unwrap(),
            };
            let last_digit = match last.parse::<i32>() {
                Ok(num) => num,
                Err(_) => *SPELLED_DIGIT_2_DIGIT.get(last).unwrap(),
            };

            // concatenate the first and last digit
            let num = format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap();

            return num;
        })
        .sum();
}

fn main() {
    let input = include_str!("../../data/input.txt");
    let output = part2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            eighthree
            sevenine",
        );
        assert_eq!(result, 281);
    }
}
