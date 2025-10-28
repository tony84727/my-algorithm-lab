pub struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut answer = 0;
        let mut prefix = 0;
        for n in nums.into_iter() {
            if n == 0 {
                let right = sum - prefix;
                if prefix == sum - prefix {
                    answer += 2;
                } else if (right - prefix).abs() <= 1 {
                    answer += 1;
                }
            }
            prefix += n;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,2,0,3] => 2; "example 1")]
    #[test_case(vec![2,3,4,0,4,1,0] => 0; "example 2")]
    #[test_case(vec![16,13,10,0,0,0,10,6,7,8,7] => 3; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::count_valid_selections(nums)
    }
}
