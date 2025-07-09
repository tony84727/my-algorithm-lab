pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut max_gap = 0;
        let mut prefix_sum = vec![0; start_time.len() + 1];
        for (i, (start, end)) in start_time.iter().zip(end_time.iter()).enumerate() {
            prefix_sum[i + 1] = prefix_sum[i] + (end - start);
        }
        for i in (k - 1)..start_time.len() {
            let left = if i == k - 1 { 0 } else { end_time[i - k] };
            let right = if i == start_time.len() - 1 {
                event_time
            } else {
                start_time[i + 1]
            };
            let length = right - left - (prefix_sum[i + 1] - prefix_sum[i + 1 - k]);
            max_gap = max_gap.max(length);
        }
        max_gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5, 1, vec![1,3], vec![2,5] => 2; "example 1")]
    #[test_case(10, 1, vec![0,2,9], vec![1,4,10] => 6; "example 2")]
    #[test_case(5, 2, vec![0,1,2,3,4], vec![1,2,3,4,5] => 0; "example 3")]
    fn test_solution(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        Solution::max_free_time(event_time, k, start_time, end_time)
    }
}
