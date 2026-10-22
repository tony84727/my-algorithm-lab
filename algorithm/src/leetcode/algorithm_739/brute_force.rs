pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        temperatures
            .iter()
            .enumerate()
            .map(|(d, v)| {
                for (n, other) in temperatures.iter().enumerate().skip(d + 1) {
                    if other > v {
                        return (n - d) as i32;
                    }
                }
                0
            })
            .collect()
    }
}
