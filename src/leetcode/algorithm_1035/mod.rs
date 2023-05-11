pub mod recursive;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,4,2], vec![1,2,4] => 2; "example 1")]
    fn test_recursive(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        recursive::Solution::max_uncrossed_lines(nums1, nums2)
    }
}
