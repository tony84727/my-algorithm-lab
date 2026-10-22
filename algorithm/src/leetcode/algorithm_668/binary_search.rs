pub struct Solution;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let count = |row: i32, x: i32| {
            let i = x / row;
            if i > n {
                n
            } else {
                i
            }
        };
        let mut start = 1;
        let mut end = m * n;
        let mut answer = 0;
        while start <= end {
            let mid = (start + end) / 2;
            let mut sum = 0;
            for row in 1..=m {
                sum += count(row, mid);
            }
            if sum < k {
                start = mid + 1;
            } else {
                end = mid - 1;
                answer = mid
            }
        }
        answer
    }
}
