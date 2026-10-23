pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut max = 0;
        let k = k as usize;
        for r in 0..k {
            let mut memoized = vec![vec![None; k]; nums.len()];
            let length = Self::solve(&mut memoized, &nums, 0, k, r, None);
            max = max.max(length);
        }
        max
    }

    fn solve(
        memoized: &mut [Vec<Option<i32>>],
        nums: &[i32],
        i: usize,
        k: usize,
        m: usize,
        last: Option<usize>,
    ) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        match last {
            Some(last) => {
                if let Some(cached) = memoized[i][last] {
                    return cached;
                }
                let answer = if (last + nums[i] as usize) % k != m {
                    Self::solve(memoized, nums, i + 1, k, m, Some(last))
                } else {
                    1 + Self::solve(memoized, nums, i + 1, k, m, Some(nums[i] as usize % k))
                };
                memoized[i][last] = Some(answer);
                answer
            }
            None => (1 + Self::solve(memoized, nums, i + 1, k, m, Some(nums[i] as usize % k)))
                .max(Self::solve(memoized, nums, i + 1, k, m, None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4,5], 2 => 5; "example 1")]
    #[test_case(vec![1,4,2,3,1,4], 3 => 4; "example 2")]
    #[test_case(vec![1,7,9], 10 => 2; "case 1")]
    #[test_case(vec![8,2,8], 5 => 3; "case 2")]
    #[test_case(vec![1,2,3,10,2], 6 => 3; "case 3")]
    fn test_solution(nums: Vec<i32>, k: i32) -> i32 {
        Solution::maximum_length(nums, k)
    }
}
