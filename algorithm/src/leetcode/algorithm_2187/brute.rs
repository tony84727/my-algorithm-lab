pub struct Solution;

impl Solution {
    pub fn minimum_time(mut time: Vec<i32>, mut total_trips: i32) -> i64 {
        let mut current_time = 1;
        time.sort_unstable();
        loop {
            for n in time.iter() {
                if current_time % *n as i64 == 0 {
                    total_trips -= 1;
                }
                if total_trips == 0 {
                    return current_time;
                }
            }
            current_time += 1;
        }
    }
}
