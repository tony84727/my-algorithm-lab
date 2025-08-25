pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat.first().unwrap().len();
        let mut flatten = Vec::with_capacity(m * n);
        let transponsed = m > n;
        let mut up = !transponsed;
        let mut row = 0;
        let mut column = 0;
        while flatten.len() < m * n {
            if column >= row && column - row < m.max(n) {
                if transponsed {
                    flatten.push(mat[column - row][row]);
                } else {
                    flatten.push(mat[row][column - row]);
                }
            }
            if up {
                if row == 0 {
                    up = !up;
                    column += 1;
                } else {
                    row -= 1;
                }
            } else if row == (if transponsed { n } else { m }) - 1 {
                up = !up;
                column += 1;
            } else {
                row += 1
            }
        }
        flatten
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,2,3],[4,5,6],[7,8,9]] => vec![1,2,4,7,5,3,6,8,9]; "example 1")]
    #[test_case(vecvec![[1,2], [3,4]] => vec![1,2,3,4]; "example 2")]
    #[test_case(vecvec![[1,2], [3,4], [5,6], [7,8]] => vec![1,2,3,5,4,6,7,8]; "case 1")]
    fn test_solution(mat: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::find_diagonal_order(mat)
    }
}
