pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut end = 0;
        temperatures
            .iter()
            .enumerate()
            .map(|(d, temperature)| {
                if end <= d || temperatures[end] <= *temperature {
                    for _ in d + 1..temperatures.len() {
                        end += 1;
                        if temperatures[end] > *temperature {
                            return (end - d) as i32;
                        }
                    }
                }
                for back in (d + 1..end).rev() {
                    if temperatures[back] <= *temperature
                        && temperatures[back - 1] > temperatures[back]
                    {
                        end = back;
                        return (back + 1 - d) as i32;
                    }
                }
                return 0;
            })
            .collect()
    }
}
