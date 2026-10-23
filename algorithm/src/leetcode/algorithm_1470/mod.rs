pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, _n: i32) -> Vec<i32> {
        let x = &nums[..nums.len() / 2];
        let y = &nums[nums.len() / 2..];
        let mut out = Vec::with_capacity(nums.len());
        for (x, y) in x.iter().zip(y.iter()) {
            out.push(*x);
            out.push(*y);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,5,1,3,4,7], 3 => vec![2,3,5,4,1,7])]
    fn test_solution(nums: Vec<i32>, n: i32) -> Vec<i32> {
        Solution::shuffle(nums, n)
    }
}
