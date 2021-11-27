pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        fn accumulate<'a, I: Iterator<Item = &'a i32>>(iterator: I) -> Vec<i32> {
            let mut accumulate = vec![];
            let mut current = 1;
            for n in iterator {
                current *= n;
                accumulate.push(current);
            }
            accumulate
        }
        let left_accumulate = accumulate(nums.iter());
        let right_accumulate = accumulate(nums.iter().rev());
        let mut answer = vec![];
        for i in 0..nums.len() {
            let left = if i == 0 { 1 } else { left_accumulate[i - 1] };
            let right = if nums.len() - i < 2 {
                1
            } else {
                right_accumulate[nums.len() - i - 2]
            };
            answer.push(left * right)
        }
        answer
    }
}
