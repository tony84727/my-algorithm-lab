pub struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        if mat.is_empty() {
            return 0;
        }
        let row = mat.len();
        let first_row = mat.first().unwrap();
        let column = first_row.len();
        let mut widths = vec![vec![0; column]; row];
        for (r, row) in mat.iter().enumerate() {
            let mut count = 0;
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    count += 1;
                } else {
                    count = 0;
                }
                widths[r][c] = count;
            }
        }
        let mut count = 0;
        for (r, row_widths) in widths.iter().enumerate() {
            for (c, _) in row_widths.iter().enumerate() {
                let mut max_height = 0;
                let mut min_width = usize::MAX;
                while r + max_height < row && widths[r + max_height][c] != 0 {
                    min_width = min_width.min(widths[r + max_height][c]);
                    count += min_width;
                    max_height += 1;
                }
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[1,0,1], [1,1,0], [1,1,0]] => 13; "example 1")]
    #[test_case(vecvec![[0,1,1,0], [0,1,1,1], [1,1,1,0]] => 24; "example 2")]
    #[test_case(vecvec![[1,1,1,1,0], [1,0,0,1,0], [0,0,1,0,1], [0,1,0,0,0]] => 17; "case 1")]
    fn test_solution(mat: Vec<Vec<i32>>) -> i32 {
        Solution::num_submat(mat)
    }
}
