use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let mut skipped_row = HashSet::new();
        let mut skipped_column = HashSet::new();

        for (y, row) in matrix.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    skipped_row.insert(y);
                    skipped_column.insert(x);
                }
            }
        }
        for &row in skipped_row.iter() {
            matrix[row].fill(0);
        }
        for &column in skipped_column.iter() {
            for row in matrix.iter_mut() {
                row[column] = 0;
            }
        }
    }
}
