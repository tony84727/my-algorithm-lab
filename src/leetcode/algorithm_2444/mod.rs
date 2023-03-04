pub struct Solution;

impl Solution {
    fn subarray_combination(nums: &[i32], min_k: i32, max_k: i32) -> i64 {
        let mut count = 0;
        for left in 0..nums.len() {
            let mut min = 0;
            let mut max = 0;
            for &add in nums.iter().skip(left) {
                if add == min_k {
                    min += 1;
                }
                if add == max_k {
                    max += 1;
                }
                if min > 0 && max > 0 {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut last = None;
        let mut segments: Vec<&[i32]> = vec![];
        for (i, &n) in nums.iter().enumerate() {
            if n > max_k || n < min_k {
                if let Some(last) = last {
                    segments.push(&nums[last..i]);
                }
                last = Some(i + 1);
            }
            if last.is_none() {
                last = Some(i);
            }
        }
        segments.push(match last {
            Some(last) => &nums[last..],
            None => &nums[..],
        });
        segments
            .into_iter()
            .map(|segment| Self::subarray_combination(segment, min_k, max_k))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,3,5,2,7,5], 1,5 => 2; "example 1")]
    #[test_case(vec![1,1,1,1],1,1 => 10; "example 2")]
    #[test_case(vec![934372,927845,479424,49441,17167,17167,65553,927845,17167,927845,17167,425106,17167,927845,17167,927845,251338,17167], 17167, 927845 => 118; "case 1")]
    #[test_case(vec![9,4,2], 2, 4 => 1; "case 2")]
    fn test_solution(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        Solution::count_subarrays(nums, min_k, max_k)
    }
}
