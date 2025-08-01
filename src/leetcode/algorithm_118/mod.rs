pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut rows: Vec<Vec<i32>> = Vec::with_capacity(num_rows);
        for r in 1..=num_rows {
            if r == 1 {
                rows.push(vec![1]);
                continue;
            }
            let mut row = Vec::with_capacity(r);
            let last_row = &rows[r - 2];
            for i in 0..r {
                if i == 0 || i == r - 1 {
                    row.push(1);
                    continue;
                }
                row.push(last_row[i - 1] + last_row[i]);
            }
            rows.push(row);
        }
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(5 => vecvec![[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]; "example 1")]
    #[test_case(1 => vec![vec![1]]; "example 2")]
    fn test_solution(num_rows: i32) -> Vec<Vec<i32>> {
        Solution::generate(num_rows)
    }
}
