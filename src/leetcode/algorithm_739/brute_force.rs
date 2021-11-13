pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        temperatures
            .iter()
            .enumerate()
            .map(|(d, v)| {
                for n in d + 1..temperatures.len() {
                    if temperatures[n] > *v {
                        return (n - d) as i32;
                    }
                }
                return 0;
            })
            .collect()
    }
}
