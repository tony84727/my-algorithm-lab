pub struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(Self::get_sum).sum()
    }

    fn get_sum(n: i32) -> i32 {
        let mut sum = n + 1;
        let mut count = 2;
        for i in 2..=n.isqrt() {
            if n % i == 0 {
                if n == i * i {
                    return 0;
                }
                count += 2;
                sum += i + n / i;
            }
            if count > 4 {
                return 0;
            }
        }
        if count == 4 {
            sum
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![21,4,7] => 32; "example 1")]
    #[test_case(vec![21,21] => 64; "example 2")]
    #[test_case(vec![1,2,3,4,5] => 0; "example 3")]
    #[test_case(vec![1,2,3,4,5,6,7,8,9,10] => 45; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::sum_four_divisors(nums)
    }
}
