pub struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut mat = vec![vec![0; n]; n];
        for query in queries.into_iter() {
            let row_start = query[0] as usize;
            let column_start = query[1] as usize;
            let row_end = query[2] as usize;
            let column_end = query[3] as usize;
            for row in mat.iter_mut().take(row_end + 1).skip(row_start) {
                for cell in row.iter_mut().take(column_end + 1).skip(column_start) {
                    *cell += 1;
                }
            }
        }
        mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(3, vecvec![[1,1,2,2],[0,0,1,1]] => vecvec![[1,1,0],[1,2,1],[0,1,1]]; "example 1")]
    #[test_case(2, vec![vec![0,0,1,1]] => vecvec![[1,1], [1,1]]; "example 2")]
    fn test_solution(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::range_add_queries(n, queries)
    }
}
