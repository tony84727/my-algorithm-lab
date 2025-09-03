pub struct Solution;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for (i, a) in points.iter().enumerate() {
            'search: for (j, b) in points.iter().enumerate() {
                if i == j {
                    continue;
                }
                if a[0] > b[0] || a[1] < b[1] {
                    continue;
                }
                for (c, inside) in points.iter().enumerate() {
                    if c == i || c == j {
                        continue;
                    }
                    if inside[0] >= a[0]
                        && inside[0] <= b[0]
                        && inside[1] >= b[1]
                        && inside[1] <= a[1]
                    {
                        continue 'search;
                    }
                }
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vecvec![[1,1],[2,2],[3,3]] => 0; "example 1")]
    #[test_case(vecvec![[6,2],[4,4],[2,6]] => 2; "example 2")]
    #[test_case(vecvec![[3,1],[1,3],[1,1]] => 2; "example 3")]
    #[test_case(vecvec![[0,1],[1,3],[6,1]] => 2; "case 1")]
    fn test_solution(points: Vec<Vec<i32>>) -> i32 {
        Solution::number_of_pairs(points)
    }
}
