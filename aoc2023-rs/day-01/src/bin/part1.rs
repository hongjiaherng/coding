fn part1(input: &str) -> i32 {
    // Get the first integer and last integer of each line, and sum them up across all lines
    return input
        .lines()
        .map(|line| {
            // Find first digit
            let first_digit = line
                .chars()
                .find(|c: &char| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();

            // Find last digit
            let last_digit = line
                .chars()
                .rev()
                .find(|c: &char| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();

            // Concatenate the first and last digit
            let num = format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap();

            return num;
        })
        .sum();
}

fn main() {
    let input = include_str!("../../data/input.txt");
    let output = part1(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
