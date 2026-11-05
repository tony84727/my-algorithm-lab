pub struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut count = 0;
        'outer: for x in arr1.into_iter() {
            for y in arr2.iter() {
                if x.abs_diff(*y) as i32 <= d {
                    continue 'outer;
                }
            }
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4, 5, 8], vec![10, 9, 1, 8], 2 => 2; "example 1")]
    #[test_case(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3 => 2; "example 2")]
    #[test_case(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6 => 1; "example 3")]
    fn test_find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        Solution::find_the_distance_value(arr1, arr2, d)
    }
}
