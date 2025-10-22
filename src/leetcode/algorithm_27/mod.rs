pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut to_remove = Vec::new();
        for (i, &n) in nums.iter().enumerate().rev() {
            if n == val {
                to_remove.push(i)
            }
        }
        for i in to_remove {
            nums.remove(i);
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,2,2,3], 3 => 2; "example 1")]
    #[test_case(vec![0,1,2,2,3,0,4,2], 2 => 5; "example 2")]
    fn test_solution(mut nums: Vec<i32>, val: i32) -> i32 {
        Solution::remove_element(&mut nums, val)
    }
}
