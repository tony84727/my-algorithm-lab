pub mod cheat;
pub mod quicksort;

#[cfg(test)]
mod tests {
    use super::quicksort;
    use quickcheck::quickcheck;

    quickcheck! {
        fn test_quicksort_solution(nums: Vec<i32>) -> bool {
            let mut expected = nums.clone();
            expected.sort_unstable();
            expected ==
            quicksort::Solution::sort_array(nums)
        }
    }
}
