use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct PartNumber {
    number: u32,
    start_idx: usize,
    end_idx: usize,
}

fn get_all_part_numbers(row: &Vec<char>) -> Vec<PartNumber> {
    // get the start idx and end idx for all numbers in the row of row_idx
    // {number: xxx, start_idx: x, end_idx: x}
    let mut part_numbers = Vec::new();
    let mut cur_part_number_str = String::new();

    let mut cur_start_idx: usize = 0;

    for (i, &item) in row.iter().enumerate() {
        if item.is_digit(10) {
            if cur_part_number_str.is_empty() {
                cur_start_idx = i;
            }
            cur_part_number_str.push(item);
        } else if !cur_part_number_str.is_empty() {
            part_numbers.push(PartNumber {
                number: cur_part_number_str.parse::<u32>().unwrap(),
                start_idx: cur_start_idx,
                end_idx: i - 1, // the end idx is the previous index
            });
            cur_part_number_str.clear();
            cur_start_idx = 0;
        }
    }

    if !cur_part_number_str.is_empty() {
        part_numbers.push(PartNumber {
            number: cur_part_number_str.parse::<u32>().unwrap(),
            start_idx: cur_start_idx,
            end_idx: row.len() - 1,
        });

        cur_part_number_str.clear();
    }

    return part_numbers;
}

fn pad_with_dot(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut padded_grid = grid.clone();

    // pad 1 slot around the whole grid
    padded_grid.insert(0, vec!['.'; padded_grid[0].len()]);
    padded_grid.insert(padded_grid.len(), vec!['.'; padded_grid[0].len()]);

    for row in padded_grid.iter_mut() {
        row.insert(0, '.');
        row.insert(row.len(), '.');
    }

    return padded_grid;
}

fn part2(input: &str) -> u32 {
    // read schematic into a 2D array
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect()) // trim any possible whitespace around
        .collect();
    // pad the grid with "." surrounding it
    grid = pad_with_dot(&grid);

    // get coord of all gear (x, y)
    let gear_coords: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row_items)| {
            row_items
                .iter()
                .enumerate()
                .filter(|(_, col_item)| col_item == &&'*')
                .map(move |(col_idx, _)| (row_idx, col_idx))
        })
        .collect();

    // find adjacent part numbers of each gear
    // let gear_part_numbers: Vec<((usize, usize), Vec<u32>)>
    let gear_part_numbers: HashMap<&(usize, usize), Vec<u32>> = gear_coords
        .iter()
        .map(|gear_coord| {
            // get adjacent part numbers for this gear
            // get the box coords around the gear *
            let box_coords: Vec<(i32, i32)> = (vec![-1, 0, 1] as Vec<i32>)
                .iter()
                .flat_map(|drow| {
                    (vec![-1, 0, 1] as Vec<i32>)
                        .iter()
                        .filter(|dcol| !(*drow == 0 && **dcol == 0))
                        .map(|dcol| (gear_coord.0 as i32 + drow, gear_coord.1 as i32 + dcol))
                        .collect::<Vec<(i32, i32)>>()
                })
                .collect();

            // all the adjacent part_numbers to this gear
            let part_numbers: Vec<u32> = box_coords
                .iter()
                .filter(|(row_idx, col_idx)| {
                    grid[*row_idx as usize][*col_idx as usize].is_digit(10)
                })
                .flat_map(|(row_idx, col_idx)| {
                    // get the full part number
                    get_all_part_numbers(&grid[*row_idx as usize]) // get all part_numbers of that row
                        .iter()
                        .filter(|&part_num| {
                            // filter those that are adjacent to the current gear
                            col_idx <= &(part_num.end_idx as i32)
                                && col_idx >= &(part_num.start_idx as i32)
                        })
                        .map(|adj_part_num| adj_part_num.number)
                        .collect::<Vec<u32>>()
                })
                .collect::<HashSet<u32>>()
                .iter()
                .cloned()
                .collect();

            return (gear_coord, part_numbers);
        })
        .collect();

    // return only gears with 2 adjacent part numbers
    return gear_part_numbers
        .iter()
        .filter(|(_, part_numbers)| part_numbers.len() == 2)
        .map(|(_, part_numbers)| part_numbers.iter().product::<u32>())
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
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let result = part2(input);

        assert_eq!(result, 467835);
    }
}

// find gear -> (return gear's coord)
// draw box around gear to find the adjacent part number -> (return all part numbers adjacent to the gear)
// return only gears with 2 adjacent part numbers -> (return gear ratio (i.e., product) of the 2 part numbers)
