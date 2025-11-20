pub struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| {
            if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });

        let mut res = 0;
        let mut p1 = -1;
        let mut p2 = -1;

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];

            if start > p2 {
                res += 2;
                p1 = end - 1;
                p2 = end;
            } else if start > p1 {
                res += 1;
                p1 = p2;
                p2 = end;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::vecvec;

    use super::*;
    use test_case::test_case;

    #[test_case(vecvec![[1,3],[3,7],[8,9]] => 5; "example 1")]
    #[test_case(vecvec![[1,3],[1,4],[2,5],[3,5]] => 3; "example 2")]
    #[test_case(vecvec![[1,2],[2,3],[2,4],[4,5]] => 5; "example 3")]
    fn test_solution(intervals: Vec<Vec<i32>>) -> i32 {
        Solution::intersection_size_two(intervals)
    }
}
