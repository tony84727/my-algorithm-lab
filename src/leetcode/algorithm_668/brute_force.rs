pub struct Solution;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut numbers = vec![];
        for i in 1..=m {
            for j in 1..=n {
                numbers.push(i * j);
            }
        }
        numbers.sort_unstable();
        numbers[k as usize - 1]
    }
}
