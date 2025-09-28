pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.reverse();
        for (ia, &a) in nums.iter().enumerate() {
            for (ib, &b) in nums.iter().enumerate().skip(ia + 1) {
                let Some(&c) = nums.get(ib + 1) else {
                    continue;
                };
                if a + c > b && b + c > a {
                    return a + b + c;
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,1,2] => 5; "example 1")]
    #[test_case(vec![1,2,1,10] => 0; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::largest_perimeter(nums)
    }
}
