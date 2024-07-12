use std::collections::HashMap;

fn get_surrounding_positions(
    row_idx: i32,
    col_idx: i32,
    grid_h: i32,
    grid_w: i32,
) -> Vec<(i32, i32)> {
    let mut surroundings: Vec<(i32, i32)> = Vec::new();

    let offsets = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    for (dx, dy) in offsets {
        let new_x = row_idx + dx;
        let new_y = col_idx + dy;

        // check if the new position is within the bound
        if new_x < grid_h && new_x >= 0 && new_y < grid_w && new_y >= 0 {
            surroundings.push((new_x, new_y));
        }
    }

    return surroundings;
}

fn is_part_number(grid: &Vec<Vec<char>>, candidate_pos: &HashMap<&str, i32>) -> bool {
    // adjacent condition: left, right, top, bottom is number or out of bound or actual symbol
    let col_start = *candidate_pos.get("col_start").unwrap();
    let col_end = *candidate_pos.get("col_end").unwrap();
    let row_idx = *candidate_pos.get("row_idx").unwrap();

    let grid_h = grid.len() as i32;
    let grid_w = grid.get(0).unwrap().len() as i32;

    for col_idx in col_start..=col_end {
        let surrounding_positions = get_surrounding_positions(row_idx, col_idx, grid_h, grid_w);

        for (i, j) in surrounding_positions {
            let surround_char = grid.get(i as usize).unwrap().get(j as usize).unwrap();
            if surround_char.is_ascii_punctuation() && surround_char != &'.' {
                return true;
            }
        }
    }
    return false;
}

fn part1(input: &str) -> u32 {
    // read schematic into a 2D array
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect()) // trim any possible whitespace around
        .collect();

    let mut part_numbers: Vec<u32> = Vec::new();

    let mut cand_part_str = String::new();
    let mut cand_part_pos = HashMap::from([("col_start", -1), ("col_end", -1), ("row_idx", -1)]);

    for (i, row_arr) in grid.iter().enumerate() {
        for (j, col_val) in row_arr.iter().enumerate() {
            if col_val.is_digit(10) {
                if cand_part_str.is_empty() {
                    cand_part_pos.insert("row_idx", i as i32);
                    cand_part_pos.insert("col_start", j as i32);
                }
                cand_part_str.push(*col_val);

                if j == row_arr.len() - 1 {
                    cand_part_pos.insert("col_end", j as i32);

                    if is_part_number(&grid, &cand_part_pos) {
                        part_numbers.push(cand_part_str.parse::<u32>().unwrap());
                    }

                    cand_part_str.clear();
                }
            } else {
                if !cand_part_str.is_empty() {
                    cand_part_pos.insert("col_end", j as i32 - 1);

                    if is_part_number(&grid, &cand_part_pos) {
                        part_numbers.push(cand_part_str.parse::<u32>().unwrap());
                    }

                    cand_part_str.clear();
                }
            }
        }
    }
    // println!("{:?}", part_numbers);
    return part_numbers.iter().sum();
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
            "467..114..
                    ...*......
                    ..35..633.
                    ......#...
                    617*......
                    .....+.58.
                    ..592.....
                    ......755.
                    ...$.*....
                    .664.598..",
        );
        assert_eq!(result, 4361);
    }
}

// build a 2D array of the engine schematic
// - with that we can get the col indices of each number (start & end) and check around the adjacency to a symbol (left, right, top, down, diagonal)
