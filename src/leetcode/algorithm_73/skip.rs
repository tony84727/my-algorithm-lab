use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut skipped_row = HashSet::new();
        let mut skipped_column = HashSet::new();

        let width = matrix.first().map(|x| x.len()).unwrap_or(0);
        for y in 0..matrix.len() {
            for x in 0..width {
                if matrix[y][x] == 0 {
                    skipped_row.insert(y);
                    skipped_column.insert(x);
                }
            }
        }
        for &row in skipped_row.iter() {
            matrix[row].fill(0);
        }
        for &column in skipped_column.iter() {
            for y in 0..matrix.len() {
                matrix[y][column] = 0;
            }
        }
    }
}
