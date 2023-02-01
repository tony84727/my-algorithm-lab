pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        fn solve(max: i32, nums: &[i32]) -> (i32, i32) {
            if nums.is_empty() {
                return (max, 0);
            }
            let (&last, rest) = nums.split_last().unwrap();
            let (max, rest_max) = solve(max, rest);
            let local_max = (rest_max + last).max(last);
            println!("{nums:?}, local_max = {local_max}, new_element = {last}");
            (if local_max > max { local_max } else { max }, local_max)
        }
        solve(i32::MIN, &nums).0
    }
}
