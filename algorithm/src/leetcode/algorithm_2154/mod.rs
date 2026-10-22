pub struct Solution;

impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        nums.sort_unstable();
        for n in nums.into_iter() {
            if n < original {
                continue;
            }
            if n == original {
                original *= 2;
                continue;
            }
            return original;
        }
        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![5,3,6,1,12], 3 => 24; "example 1")]
    #[test_case(vec![2,7,9], 4 => 4; "example 2")]
    #[test_case(vec![4,7,1,16,1,2,7,13], 1 => 8; "case 1")]
    #[test_case(vec![161,28,640,264,81,561,320,2,61,244,183,108,773,61,976,122,988,2,370,392,488,375,349,432,713,563], 61 => 1952; "case 2")]
    fn test_solution(nums: Vec<i32>, original: i32) -> i32 {
        Solution::find_final_value(nums, original)
    }
}
