pub mod back_trace;

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, iter::FromIterator};

    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(vec![1,2,3], vecvec![[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,2,1], [3,1,2]]; "example 1")]
    fn test_back_trace(nums: Vec<i32>, right_answer: Vec<Vec<i32>>) {
        let right_length = right_answer.len();
        let answer = back_trace::Solution::permute(nums);
        let actual_length = answer.len();
        assert_eq!(
            right_length,
            actual_length,
            "expected length to be {right_length}, got {actual_length}, elements are: {answer:?}"
        );
        let right_answer: HashSet<Vec<i32>> = HashSet::from_iter(right_answer);
        assert_eq!(right_answer, HashSet::from_iter(answer.into_iter()));
    }
}
