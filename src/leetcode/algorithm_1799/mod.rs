pub struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut sequence = Self::solve(nums);
        sequence.sort();
        Self::multiplied(&sequence)
    }

    fn solve(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![Self::gcd(nums[0], nums[1])];
        }
        let mut max = 0;
        let mut max_sequence = vec![];
        let first = nums.pop().unwrap();
        for pair_of_first in 0..nums.len() {
            let mut nums = nums.clone();
            let pair = nums.swap_remove(pair_of_first);
            let mut sequence = Self::solve(nums.clone());
            sequence.push(Self::gcd(first, pair));
            sequence.sort();
            let sum = Self::multiplied(&sequence);
            if sum > max {
                max = sum;
                max_sequence = sequence;
            }
        }
        max_sequence
    }

    fn multiplied(nums: &[i32]) -> i32 {
        nums.iter()
            .enumerate()
            .map(|(i, a)| (i + 1) as i32 * a)
            .sum()
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 && b != 0 {
            if a > b {
                a -= b;
            } else {
                b -= a;
            }
        }
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2] => 1; "example 1")]
    #[test_case(vec![3,4,6,8] => 11; "example 2")]
    #[test_case(vec![1,2,3,4,5,6] => 14; "example 3")]
    #[test_case(vec![9,2] => 1; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::max_score(nums)
    }

    #[test_case(1, 3 => 1)]
    #[test_case(3, 4 => 1)]
    #[test_case(9, 12 => 3)]
    fn test_gcd(a: i32, b: i32) -> i32 {
        Solution::gcd(a, b)
    }
}
