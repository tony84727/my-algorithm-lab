use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let all_possible = HashSet::from_iter(1..=9_i32);
        let mut options: Vec<Vec<HashSet<i32>>> = vec![vec![all_possible; 9]; 9];
        let mut empty = 0;
        for (r, row) in board.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == '.' {
                    empty += 1;
                    continue;
                }
                options[r][c].clear();
                let digit = cell.to_digit(10).unwrap() as i32;
                for set in options[r].iter_mut() {
                    set.remove(&digit);
                }
                for column_sets in options.iter_mut() {
                    column_sets[c].remove(&digit);
                }
                let grid_row = r / 3;
                let grid_column = c / 3;
                for row in options.iter_mut().skip(grid_row * 3).take(3) {
                    for cell in row.iter_mut().skip(grid_column * 3).take(3) {
                        cell.remove(&digit);
                    }
                }
            }
        }
        Self::try_solve(&mut options, empty, board);
    }

    fn try_solve(options: &mut [Vec<HashSet<i32>>], empty: usize, board: &mut [Vec<char>]) -> bool {
        if empty == 0 {
            return true;
        }
        let mut min = 11;
        let mut r = 0;
        let mut c = 0;
        for (ir, row_options) in options.iter().enumerate() {
            for (ic, options) in row_options.iter().enumerate() {
                if board[ir][ic] != '.' {
                    continue;
                }
                if options.len() < min {
                    min = options.len();
                    r = ir;
                    c = ic;
                }
            }
        }

        let origin_options = std::mem::take(&mut options[r][c]);
        for &option in origin_options.iter() {
            let mut modified: Vec<(usize, usize)> = Vec::new();
            board[r][c] = Self::to_digit(option);
            for (c, set) in options[r].iter_mut().enumerate() {
                if set.remove(&option) {
                    modified.push((r, c));
                }
            }
            for (r, column_sets) in options.iter_mut().enumerate() {
                if column_sets[c].remove(&option) {
                    modified.push((r, c));
                }
            }
            let grid_row = r / 3;
            let grid_column = c / 3;
            for (r, row) in options.iter_mut().enumerate().skip(grid_row * 3).take(3) {
                for (c, cell) in row.iter_mut().enumerate().skip(grid_column * 3).take(3) {
                    if cell.remove(&option) {
                        modified.push((r, c));
                    }
                }
            }
            if Self::try_solve(options, empty - 1, board) {
                return true;
            }
            board[r][c] = '.';
            for (r, c) in modified.into_iter() {
                options[r][c].insert(option);
            }
        }
        options[r][c] = origin_options;
        false
    }

    fn to_digit(i: i32) -> char {
        (b'0' + i as u8) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(
        vecvec![
            ['5','3','.','.','7','.','.','.','.'],
            ['6','.','.','1','9','5','.','.','.'],
            ['.','9','8','.','.','.','.','6','.'],
            ['8','.','.','.','6','.','.','.','3'],
            ['4','.','.','8','.','3','.','.','1'],
            ['7','.','.','.','2','.','.','.','6'],
            ['.','6','.','.','.','.','2','8','.'],
            ['.','.','.','4','1','9','.','.','5'],
            ['.','.','.','.','8','.','.','7','9']
        ],
        vecvec![
            ['5','3','4','6','7','8','9','1','2'],
            ['6','7','2','1','9','5','3','4','8'],
            ['1','9','8','3','4','2','5','6','7'],
            ['8','5','9','7','6','1','4','2','3'],
            ['4','2','6','8','5','3','7','9','1'],
            ['7','1','3','9','2','4','8','5','6'],
            ['9','6','1','5','3','7','2','8','4'],
            ['2','8','7','4','1','9','6','3','5'],
            ['3','4','5','2','8','6','1','7','9']
        ]; "example 1")]
    #[test_case(
        vecvec![
            ['.','.','9','7','4','8','.','.','.'],
            ['7','.','.','.','.','.','.','.','.'],
            ['.','2','.','1','.','9','.','.','.'],
            ['.','.','7','.','.','.','2','4','.'],
            ['.','6','4','.','1','.','5','9','.'],
            ['.','9','8','.','.','.','3','.','.'],
            ['.','.','.','8','.','3','.','2','.'],
            ['.','.','.','.','.','.','.','.','6'],
            ['.','.','.','2','7','5','9','.','.']
        ],
        vecvec![
            ['5','1','9','7','4','8','6','3','2'],
            ['7','8','3','6','5','2','4','1','9'],
            ['4','2','6','1','3','9','8','7','5'],
            ['3','5','7','9','8','6','2','4','1'],
            ['2','6','4','3','1','7','5','9','8'],
            ['1','9','8','5','2','4','3','6','7'],
            ['9','7','5','8','6','3','1','2','4'],
            ['8','3','2','4','9','1','7','5','6'],
            ['6','4','1','2','7','5','9','8','3']
        ]; "case 1")]
    fn test_solution(mut input: Vec<Vec<char>>, expected: Vec<Vec<char>>) {
        Solution::solve_sudoku(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn test_to_digit() {
        assert_eq!('9', Solution::to_digit(9));
        assert_eq!('1', Solution::to_digit(1));
    }
}
