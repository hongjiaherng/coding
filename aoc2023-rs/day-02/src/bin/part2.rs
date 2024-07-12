use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    return input
        .lines()
        .map(|line| {
            // list of key-value pairs of color: number
            let game_record: Vec<HashMap<&str, i32>> = line
                .split(":")
                .last()
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

            // get the max of each color in the form of hashmap
            let mut min_config: HashMap<&str, i32> = HashMap::new();
            for record in game_record {
                for color in record.keys() {
                    let min_config_count = min_config.get(color).unwrap_or(&0);
                    let cur_record_count = record.get(color).unwrap();

                    if cur_record_count > min_config_count {
                        min_config.insert(color, *cur_record_count);
                    }
                }
            }

            let mul: i32 = min_config.values().cloned().product();
            return mul;
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
        assert_eq!(result, 2286);
    }
}
