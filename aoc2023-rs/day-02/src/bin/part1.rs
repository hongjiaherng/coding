use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let config = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    return input
        .lines()
        .map(|line| {
            let mut line_parts = line.split(":");

            let game_id = line_parts
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            // list of key-value pairs of color: number
            let game_record: Vec<HashMap<&str, i32>> = line_parts
                .next()
                .unwrap()
                .split(";")
                .map(|kv_pairs| {
                    kv_pairs
                        .split(',')
                        .filter_map(|kv_str| {
                            let mut iter = kv_str.trim().split_whitespace();
                            let count = iter.next()?.parse().unwrap();
                            let color = iter.next()?;
                            Some((color, count))
                        })
                        .collect()
                })
                .collect();

            // Iterate over game_record
            for record in game_record {
                for color in record.keys() {
                    if record.get(color) > config.get(color) {
                        return 0;
                    }
                }
            }

            return game_id;
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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}

// cube of red / green / blue
// hide a number of cubes of each color in the bag
// figure out how many number of cubes hide

// The Elf would first like to know which games would have been possible
// if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

// Game 1: Possible
// red: 12 | green: 13 | blue: 14
// ------------------------------
// red: 4  | green: 0  | blue: 3
// red: 1  | green: 2  | blue: 6
// red: 0  | green: 2  | blue: 0

// Game 2: Possible
// red: 12 | green: 13 | blue: 14
// ------------------------------
// red: 0  | green: 2  | blue: 1
// red: 1  | green: 3  | blue: 4
// red: 0  | green: 1  | blue: 1

// Game 3: Impossible
// red: 12 | green: 13 | blue: 14
// ------------------------------
// red: (20) | green: 8  | blue: 6
// red: 4  | green: 13 | blue: 5
// red: 1  | green: 5  | blue: 0

// Game 4: Impossible
// red: 12 | green: 13 | blue: 14
// ------------------------------
// red: 3 | green: 1  | blue: 6
// red: 6  | green: 3 | blue: 0
// red: (14)  | green: 3  | blue: (15)

// Game 5: Possible
// red: 12 | green: 13 | blue: 14
// ------------------------------

// SUM(ID of possible games) = 1 + 2 + 5 = 8

// config: {"red": 12, "green": 13, "blue": 14}
