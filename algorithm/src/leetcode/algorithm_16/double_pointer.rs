use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut answer = None;
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let current = nums[i] + nums[left] + nums[right];
                match target.cmp(&current) {
                    Ordering::Less => right -= 1,
                    Ordering::Greater => left += 1,
                    Ordering::Equal => return target,
                }
                let Some(current_answer) = answer else {
                    answer = Some(current);
                    continue;
                };
                if current_answer.abs_diff(target) > current.abs_diff(target) {
                    answer = Some(current);
                }
            }
        }
        answer.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::Solution;

    #[test_case(vec![-1, 2, 1, -4], 1 => 2; "example 1")]
    #[test_case(vec![0,0,0], 1 => 0; "example 2")]
    #[test_case(vec![-1,2,1,-4], 1 => 2; "case 1")]
    #[test_case(vec![2,3,8,9,10], 16 => 15; "case 2")]
    fn test_solution(nums: Vec<i32>, target: i32) -> i32 {
        Solution::three_sum_closest(nums, target)
    }
}
