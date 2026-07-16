pub struct Solution;

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let mut prefix_max = nums.clone();
        let mut current_max = 0;
        for n in prefix_max.iter_mut() {
            if *n <= current_max {
                *n = current_max;
                continue;
            }
            current_max = *n;
        }
        let mut prefix_gcd: Vec<i32> = prefix_max
            .into_iter()
            .enumerate()
            .map(|(i, x)| Self::gcd(nums[i], x))
            .collect();
        prefix_gcd.sort_unstable();
        let mut sum = 0_i64;
        let n = prefix_gcd.len();
        for i in 0..prefix_gcd.len() / 2 {
            sum += Self::gcd(prefix_gcd[i], prefix_gcd[n - i - 1]) as i64;
        }
        sum
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,6,4] => 2; "example 1")]
    #[test_case(vec![3,6,2,8] => 5; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i64 {
        Solution::gcd_sum(nums)
    }
}
