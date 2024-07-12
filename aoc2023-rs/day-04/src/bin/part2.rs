use std::collections::HashSet;

fn part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let mut part = line.split(":").last().unwrap().split("|");
            let winning_nums: HashSet<u32> = part
                .next()
                .unwrap()
                .split_whitespace()
                .into_iter()
                .map(|cnum| cnum.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let candidate_nums: HashSet<u32> = part
                .next()
                .unwrap()
                .split_whitespace()
                .into_iter()
                .map(|cnum| cnum.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();

            let win_count = winning_nums.intersection(&candidate_nums).count() as u32;

            let score = if win_count == 0 {
                0
            } else {
                u32::pow(2, win_count - 1)
            };

            return score;
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
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
