use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // validate 3*3
        for grid_row in 0..3 {
            for grid_column in 0..3 {
                let mut numbers = HashSet::new();
                for row in board.iter().skip(grid_row * 3).take(3) {
                    for &cell in row.iter().skip(grid_column * 3).take(3) {
                        if cell == '.' {
                            continue;
                        }
                        if numbers.contains(&cell) {
                            return false;
                        }
                        numbers.insert(cell);
                    }
                }
            }
        }
        // validate rows
        for row in board.iter() {
            let mut numbers = HashSet::new();
            for &cell in row.iter() {
                if cell == '.' {
                    continue;
                }
                if numbers.contains(&cell) {
                    return false;
                }
                numbers.insert(cell);
            }
        }
        // validate columns
        for column in 0..9 {
            let mut numbers = HashSet::new();
            for row in board.iter() {
                let cell = row[column];
                if cell == '.' {
                    continue;
                }
                if numbers.contains(&cell) {
                    return false;
                }
                numbers.insert(cell);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![
        ['5','3','.','.','7','.','.','.','.'],
        ['6','.','.','1','9','5','.','.','.'],
        ['.','9','8','.','.','.','.','6','.'],
        ['8','.','.','.','6','.','.','.','3'],
        ['4','.','.','8','.','3','.','.','1'],
        ['7','.','.','.','2','.','.','.','6'],
        ['.','6','.','.','.','.','2','8','.'],
        ['.','.','.','4','1','9','.','.','5'],
        ['.','.','.','.','8','.','.','7','9']
    ] => true; "example 1")]
    #[test_case(vecvec![
        ['8','3','.','.','7','.','.','.','.'],
        ['6','.','.','1','9','5','.','.','.'],
        ['.','9','8','.','.','.','.','6','.'],
        ['8','.','.','.','6','.','.','.','3'],
        ['4','.','.','8','.','3','.','.','1'],
        ['7','.','.','.','2','.','.','.','6'],
        ['.','6','.','.','.','.','2','8','.'],
        ['.','.','.','4','1','9','.','.','5'],
        ['.','.','.','.','8','.','.','7','9']
    ] => false; "example 2")]
    fn test_solution(board: Vec<Vec<char>>) -> bool {
        Solution::is_valid_sudoku(board)
    }
}
