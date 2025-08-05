use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rowmap: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut colmap: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut subboxmap: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, &ch) in row.iter().enumerate() {
                if ch == '.' {
                    continue;
                }

                if rowmap.get(&row_idx).map_or(false, |s| s.contains(&ch))
                    || colmap.get(&col_idx).map_or(false, |s| s.contains(&ch))
                    || subboxmap
                        .get(&(row_idx / 3, col_idx / 3))
                        .map_or(false, |s| s.contains(&ch))
                {
                    return false;
                }

                rowmap.entry(row_idx).or_default().insert(ch);
                colmap.entry(col_idx).or_default().insert(ch);
                subboxmap
                    .entry((row_idx / 3, col_idx / 3))
                    .or_default()
                    .insert(ch);
            }
        }

        true
    }

    pub fn is_valid_sudoku_3passes(board: Vec<Vec<char>>) -> bool {
        // T(n) = O(9^2)
        // S(n) = O(9)
        let mut set: HashSet<char> = HashSet::new();

        // Check horizontally
        for i in 0..board.len() {
            for j in 0..board.len() {
                if set.contains(&board[i][j]) && board[i][j] != '.' {
                    return false;
                }
                set.insert(board[i][j]);
            }
            set.clear();
        }

        // Check vertically
        for i in 0..board.len() {
            for j in 0..board.len() {
                if set.contains(&board[j][i]) && board[j][i] != '.' {
                    return false;
                }
                set.insert(board[j][i]);
            }
            set.clear();
        }

        // Check sub-box wise
        for i in 0..board.len() {
            // Loop thru each 3x3 sub-boxes
            let row_forward = 3 * (i / 3);
            let col_forward = 3 * (i % 3);
            for j in 0..board.len() {
                // Loop within each 3x3 sub-boxes
                let cur_cell = board[(j / 3) + row_forward][(j % 3) + col_forward];
                if set.contains(&cur_cell) && cur_cell != '.' {
                    return false;
                }
                set.insert(cur_cell);
            }
            set.clear();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::q36_valid_sudoku::Solution;

    /// Example 1:
    /// Input: board =
    /// [["5","3",".",".","7",".",".",".","."]
    /// ,["6",".",".","1","9","5",".",".","."]
    /// ,[".","9","8",".",".",".",".","6","."]
    /// ,["8",".",".",".","6",".",".",".","3"]
    /// ,["4",".",".","8",".","3",".",".","1"]
    /// ,["7",".",".",".","2",".",".",".","6"]
    /// ,[".","6",".",".",".",".","2","8","."]
    /// ,[".",".",".","4","1","9",".",".","5"]
    /// ,[".",".",".",".","8",".",".","7","9"]]
    /// Output: true
    #[test]
    fn test_is_valid_sudoku_example_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));
    }

    /// Example 2:
    /// Input: board =
    /// [["8","3",".",".","7",".",".",".","."]
    /// ,["6",".",".","1","9","5",".",".","."]
    /// ,[".","9","8",".",".",".",".","6","."]
    /// ,["8",".",".",".","6",".",".",".","3"]
    /// ,["4",".",".","8",".","3",".",".","1"]
    /// ,["7",".",".",".","2",".",".",".","6"]
    /// ,[".","6",".",".",".",".","2","8","."]
    /// ,[".",".",".","4","1","9",".",".","5"]
    /// ,[".",".",".",".","8",".",".","7","9"]]
    /// Output: false
    /// Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
    #[test]
    fn test_is_valid_sudoku_example_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}
