pub struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        if arr.len() <= 2 {
            return true;
        }
        arr.sort_unstable();
        let incremental = arr[1] - arr[0];
        let mut last = arr[1];
        for n in arr.into_iter().skip(2) {
            if n - last != incremental {
                return false;
            }
            last = n;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,5,1] => true; "example 1")]
    #[test_case(vec![1,2,4] => false; "example 2")]
    fn test_solution(input: Vec<i32>) -> bool {
        Solution::can_make_arithmetic_progression(input)
    }
}
