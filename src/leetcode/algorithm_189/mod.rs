pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        nums.rotate_right(k as usize % l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4,5,6,7], 3 => vec![5,6,7,1,2,3,4])]
    fn test_solution(mut input: Vec<i32>, k: i32) -> Vec<i32> {
        Solution::rotate(&mut input, k);
        input
    }
}
