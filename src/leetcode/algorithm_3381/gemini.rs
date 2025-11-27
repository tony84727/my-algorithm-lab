pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut max_so_far = i64::MIN;

        if k == 0 || n < k {
            return 0;
        }

        let mut prefix_sums = vec![0i64; n + 1];
        for i in 0..n {
            prefix_sums[i + 1] = prefix_sums[i] + nums[i] as i64;
        }

        let get_chunk_sum = |i: usize| -> i64 {
            prefix_sums[i + k] - prefix_sums[i]
        };

        for j in 0..k {
            let mut sub_sums = vec![];
            let mut i = j;
            while i + k <= n {
                sub_sums.push(get_chunk_sum(i));
                i += k;
            }

            if sub_sums.is_empty() {
                continue;
            }
            
            // Kadane's algorithm
            let mut max_ending_here = sub_sums[0];
            let mut global_max_for_j = sub_sums[0];
            for i in 1..sub_sums.len() {
                let current_sum = sub_sums[i];
                max_ending_here = current_sum.max(current_sum + max_ending_here);
                global_max_for_j = global_max_for_j.max(max_ending_here);
            }
            
            if max_so_far == i64::MIN {
                max_so_far = global_max_for_j;
            } else {
                max_so_far = max_so_far.max(global_max_for_j);
            }
        }

        max_so_far
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2], 1 => 3; "example 1")]
    #[test_case(vec![-1,-2,-3,-4,-5], 4 => -10; "example 2")]
    #[test_case(vec![-5,1,2,-3,4], 2 => 4; "example 3")]
    fn test_solution(nums: Vec<i32>, k: i32) -> i64 {
        Solution::max_subarray_sum(nums, k)
    }
}