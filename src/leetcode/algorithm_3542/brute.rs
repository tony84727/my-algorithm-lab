pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        Self::solve(&mut nums)
    }

    fn solve(nums: &mut [i32]) -> i32 {
        let Some(to_remove) = nums.iter().min().cloned() else {
            return 0;
        };
        let mut count = if to_remove != 0 { 1 } else { 0 };
        let mut start = None;
        let n = nums.len();
        for i in 0..n {
            if nums[i] == to_remove {
                nums[i] = 0;
            }
            if nums[i] == 0 {
                if let Some(start) = start {
                    count += Self::solve(&mut nums[start..i]);
                }
                start = None;
                continue;
            } else if start.is_none() {
                start = Some(i);
            }
        }
        if let Some(start) = start {
            count += Self::solve(&mut nums[start..]);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,2] => 1; "example 1")]
    #[test_case(vec![3,1,2,1] => 3; "example 2")]
    #[test_case(vec![1,2,1,2,1,2] => 4; "example 3")]
    #[test_case(vec![0] => 0; "case 1")]
    #[test_case(vec![2,4,5] => 3; "case 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::min_operations(nums)
    }
}
