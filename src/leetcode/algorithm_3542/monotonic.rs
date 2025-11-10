pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = vec![0];
        let mut ops = 0;

        for value in nums {
            while stack.last().is_some_and(|&top| top > value) {
                stack.pop();
            }

            if stack.last().copied().unwrap_or(0) < value {
                stack.push(value);
                ops += 1;
            }
        }

        ops
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
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
