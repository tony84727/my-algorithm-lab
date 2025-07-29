use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut maximum: Vec<i32> = nums.clone();
        let mut bits: Vec<Vec<usize>> = vec![vec![]; 32];
        for (i, n) in nums.iter().enumerate().rev() {
            maximum[i] = n | if i + 1 == nums.len() {
                0
            } else {
                maximum[i + 1]
            };
            let mut n = *n;
            let mut digit = 0;
            while n != 0 {
                if n & 1 != 0 {
                    bits[digit].push(i);
                }
                n >>= 1;
                digit += 1;
            }
        }
        let mut count = vec![0; nums.len()];
        for (i, n) in nums.iter().enumerate() {
            let mut max_required = i;
            let mut missing = maximum[i] ^ n;
            let mut digit = 0;
            while missing != 0 {
                if missing & 1 != 0 {
                    let result = bits[digit]
                        .binary_search_by_key(&Reverse(i), |&x| Reverse(x))
                        .map(|found_index| bits[digit][found_index - 1])
                        .unwrap_or_else(|x| bits[digit][x - 1]);
                    max_required = max_required.max(result);
                }
                digit += 1;
                missing >>= 1;
            }
            count[i] = (max_required - i + 1) as i32;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,2,1,3] => vec![3,3,2,2,1]; "example 1")]
    #[test_case(vec![1,2] => vec![2,1]; "example 2")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        Solution::smallest_subarrays(nums)
    }
}
