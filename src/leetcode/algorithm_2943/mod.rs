pub struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(
        _n: i32,
        _m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        h_bars.sort_unstable();
        v_bars.sort_unstable();
        (Self::max_gap(&h_bars) + 1)
            .min(Self::max_gap(&v_bars) + 1)
            .pow(2)
    }

    fn max_gap(nums: &[i32]) -> i32 {
        let mut start = nums[0];
        let mut length = 1;
        let mut max = 1;
        for &n in nums.iter().skip(1) {
            if n == start + length {
                length += 1;
                continue;
            }
            max = max.max(length);
            start = n;
            length = 1
        }
        max.max(length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2,1,vec![2,3], vec![2] => 4; "example 1")]
    #[test_case(1,1,vec![2], vec![2] => 4; "example 2")]
    #[test_case(2,3,vec![2,3], vec![2,4] => 4; "example 3")]
    #[test_case(3,2,vec![3,2,4], vec![3,2] => 9; "case 1")]
    fn test_solution(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        Solution::maximize_square_hole_area(n, m, h_bars, v_bars)
    }
}
