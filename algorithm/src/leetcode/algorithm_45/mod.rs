pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut min_steps = vec![i32::MAX; nums.len()];
        min_steps[0] = 0;
        let max = nums.len() - 1;
        for (i, j) in nums.into_iter().enumerate() {
            let current = min_steps[i];
            for step in min_steps
                .iter_mut()
                .take((i + j as usize).min(max) + 1)
                .skip(i)
            {
                *step = (*step).min(current + 1);
            }
        }
        *min_steps.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,3,1,1,4] => 2; "example 1")]
    #[test_case(vec![2,3,0,1,4] => 2; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::jump(nums)
    }
}
