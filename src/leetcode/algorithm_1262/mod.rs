pub struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut mods = [0, 0, 0];
        for n in nums.into_iter() {
            let mut next = mods;
            for &sum in mods.iter() {
                let m = ((sum + n) % 3) as usize;
                next[m] = next[m].max(sum + n);
            }
            mods = next;
        }
        mods[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,6,5,1,8] => 18; "example 1")]
    #[test_case(vec![4] => 0; "example 2")]
    #[test_case(vec![1,2,3,4,4] => 12; "example 3")]
    #[test_case(vec![2,6,2,2,7] => 15; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::max_sum_div_three(nums)
    }
}
