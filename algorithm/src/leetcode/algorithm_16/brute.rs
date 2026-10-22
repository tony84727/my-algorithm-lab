pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut answer = None;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    let sum = nums[i] + nums[j] + nums[k];
                    let Some(current) = answer else {
                        answer = Some(sum);
                        continue;
                    };
                    if sum.abs_diff(target) < current.abs_diff(target) {
                        answer = Some(sum);
                    }
                }
            }
        }
        answer.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::algorithm_16::brute::Solution;

    #[test_case(vec![-1, 2, 1, -4], 1 => 2; "example 1")]
    #[test_case(vec![0,0,0], 1 => 0; "example 2")]
    fn test_solution(nums: Vec<i32>, target: i32) -> i32 {
        Solution::three_sum_closest(nums, target)
    }
}
