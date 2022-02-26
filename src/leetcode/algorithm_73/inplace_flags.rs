pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let width = matrix.first().map(|row| row.len()).unwrap_or(0);
        let mut first_column_zero = false;
        for row in 0..matrix.len() {
            for column in 0..width {
                if matrix[row][column] == 0 {
                    if column == 0 {
                        first_column_zero = true;
                    } else {
                        matrix[0][column] = 0;
                    }
                    matrix[row][0] = 0;
                }
            }
        }
        for row in (0..matrix.len()).rev() {
            for column in (0..width).rev() {
                let column_is_zero = if column == 0 {
                    first_column_zero
                } else {
                    matrix[0][column] == 0
                };
                if column_is_zero || matrix[row][0] == 0 {
                    matrix[row][column] = 0
                }
            }
        }
    }
}
