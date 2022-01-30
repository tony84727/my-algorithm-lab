pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn merging(intervals: &mut Vec<Vec<i32>>) {
            intervals.sort_unstable();
            let mut current = 0;
            while current < intervals.len() - 1 {
                if intervals[current][1] >= intervals[current + 1][0] {
                    intervals[current + 1][0] = intervals[current][0];
                    intervals[current + 1][1] =
                        intervals[current][1].max(intervals[current + 1][1]);
                    intervals.swap_remove(current);
                }
                current += 1;
            }
        }
        loop {
            let before = intervals.len();
            merging(&mut intervals);
            if before == intervals.len() {
                break;
            }
        }
        intervals
    }
}
