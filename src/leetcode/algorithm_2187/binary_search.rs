pub struct Solution;

impl Solution {
    fn test(time: &[i32], target: i64, n: i64) -> bool {
        let mut trip = 0;
        for interval in time.iter() {
            trip += n / *interval as i64;
            if trip >= target {
                return true;
            }
        }
        false
    }
    fn find_min_max(time: &[i32]) -> (i32, i32) {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &n in time.iter() {
            min = min.min(n);
            max = max.max(n);
        }
        (min, max)
    }
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        if time.is_empty() {
            return 0;
        }
        let total_trips = total_trips as i64;
        let (min, max) = Self::find_min_max(&time);
        let mut max = max as i64 * total_trips;
        let mut min = (min as f32 / total_trips as f32).ceil() as i64;
        let mut last = max;
        while min < max {
            let mid = min + (max - min) / 2;
            if Self::test(&time, total_trips, mid) {
                last = mid;
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        last
    }
}
