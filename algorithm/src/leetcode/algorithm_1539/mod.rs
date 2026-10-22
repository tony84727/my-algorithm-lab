pub struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut expected = 0;
        let mut missing = 0;
        'outer: for n in arr.into_iter() {
            expected += 1;
            if n == expected {
                continue;
            }
            while expected != n {
                missing += 1;
                println!("{n} {missing} {expected}");
                if missing == k {
                    break 'outer;
                }
                expected += 1;
            }
        }
        expected += k - missing;
        expected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,3,4,7,11], 5 => 9; "example 1")]
    #[test_case(vec![1,2,3,4], 2 => 6; "example 2")]
    #[test_case(vec![2], 1 => 1; "case 1")]
    #[test_case(vec![3,10], 2 => 2; "case 2")]
    fn test_solution(arr: Vec<i32>, k: i32) -> i32 {
        Solution::find_kth_positive(arr, k)
    }
}
