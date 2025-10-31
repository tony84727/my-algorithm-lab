pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];
        let mut max = 0;
        for n in nums.into_iter() {
            let r = (n as usize) % k;
            let updates: Vec<i32> = dp.iter().map(|row| row[r] + 1).collect();
            if let Some(dp_row) = dp.get_mut(r) {
                for (value, cell) in updates.into_iter().zip(dp_row.iter_mut()) {
                    *cell = value;
                    max = max.max(value);
                }
            }
        }
        max
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
