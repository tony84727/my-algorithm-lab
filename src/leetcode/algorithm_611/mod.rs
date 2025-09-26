pub struct Solution;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for (ia, &a) in nums.iter().enumerate() {
            for (ib, &b) in nums.iter().enumerate().skip(ia + 1) {
                for &c in nums.iter().skip(ib + 1) {
                    if a + b > c && c + b > a && a + c > b {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,2,3,4] => 3; "example 1")]
    #[test_case(vec![4,2,3,4] => 4; "example 2")]
    #[test_case(vec![7,0,0,0] => 0; "case 1")]
    #[test_case(vec![24,3,82,22,35,84,19] => 10; "case 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::triangle_number(nums)
    }
}
