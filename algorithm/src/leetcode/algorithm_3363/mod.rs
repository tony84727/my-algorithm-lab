pub struct Solution;

impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let mut child_0 = 0;
        let n = fruits.len();
        for (ri, row) in fruits.iter().enumerate() {
            child_0 += row[ri];
        }
        let mut dp_1 = vec![vec![0; n / 2]; n - 1];
        for move_down in 0..n - 1 {
            let get_row_column = |down: usize| -> usize {
                if down < n / 2 {
                    down + 1
                } else if !n.is_multiple_of(2) && down == n / 2 + 1 {
                    n / 2
                } else {
                    n - (down + 1)
                }
            };
            let previous = if move_down == 0 {
                1
            } else {
                get_row_column(move_down - 1)
            };
            let row_columns = get_row_column(move_down);
            for column in 0..row_columns {
                if move_down == 0 {
                    dp_1[move_down][column] = fruits[move_down][n - 1 - column];
                    continue;
                }
                let previous_row_low = (previous - 1).min(column.saturating_sub(1));
                let previous_row_high = (previous - 1).min(column + 1);
                let mut last_row_max = 0;
                for &value in dp_1[move_down - 1]
                    .iter()
                    .skip(previous_row_low)
                    .take(previous_row_high - previous_row_low + 1)
                {
                    last_row_max = last_row_max.max(value);
                }
                dp_1[move_down][column] = last_row_max + fruits[move_down][n - 1 - column];
            }
        }
        let child_1 = dp_1[n - 2][0];
        let mut dp_2 = vec![vec![0; n - 1]; n / 2];
        for move_right in 0..n - 1 {
            let get_column_row = |right: usize| -> usize {
                if right < n / 2 {
                    right + 1
                } else if !n.is_multiple_of(2) && right == n / 2 + 1 {
                    n / 2
                } else {
                    n - (right + 1)
                }
            };
            let previous = if move_right == 0 {
                1
            } else {
                get_column_row(move_right - 1)
            };
            let column_rows = get_column_row(move_right);
            for row in 0..column_rows {
                if move_right == 0 {
                    dp_2[row][move_right] = fruits[n - 1 - row][move_right];
                    continue;
                }
                let previous_row_low = (previous - 1).min(row.saturating_sub(1));
                let previous_row_high = (previous - 1).min(row + 1);
                let mut last_row_max = 0;
                for last_row in dp_2
                    .iter()
                    .skip(previous_row_low)
                    .take(previous_row_high - previous_row_low + 1)
                {
                    last_row_max = last_row_max.max(last_row[move_right - 1]);
                }
                dp_2[row][move_right] = last_row_max + fruits[n - 1 - row][move_right];
            }
        }
        let child_2 = dp_2[0][n - 2];

        child_0 + child_1 + child_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,2,3,4],[5,6,8,7],[9,10,11,12],[13,14,15,16]] => 100; "example 1")]
    #[test_case(vecvec![[1,1],[1,1]] => 4; "example 2")]
    fn test_solution(fruits: Vec<Vec<i32>>) -> i32 {
        Solution::max_collected_fruits(fruits)
    }
}
