pub struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then_with(|| b[0].cmp(&a[0])));

        let mut res = 0;
        let mut second_largest = -1;
        let mut largest = -1;

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];

            if start > largest {
                res += 2;
                second_largest = end - 1;
                largest = end;
            } else if start > second_largest {
                res += 1;
                second_largest = largest;
                largest = end;
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
