use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut max_count = 0;
        let mut kind_count = HashMap::<i32, i32>::new();
        let mut start = 0;
        let mut end = 0;
        let mut kinds = 0;
        while end < fruits.len() {
            while end < fruits.len() {
                if !kind_count.contains_key(&fruits[end]) {
                    if kinds >= 2 {
                        break;
                    } else {
                        kinds += 1;
                    }
                }
                *kind_count.entry(fruits[end]).or_default() += 1;
                end += 1;
            }
            max_count = max_count.max(end - start);
            while kinds >= 2 {
                let count = kind_count.get_mut(&fruits[start]).unwrap();
                *count -= 1;
                if *count == 0 {
                    kinds -= 1;
                    kind_count.remove(&fruits[start]);
                }
                start += 1;
            }
        }
        max_count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,1] => 3; "example 1")]
    #[test_case(vec![0,1,2,2] => 3; "example 2")]
    #[test_case(vec![1,2,3,2,2] => 4; "example 3")]
    #[test_case(vec![3,3,3,1,2,1,1,2,3,3,4] => 5; "case 1")]
    #[test_case(vec![0] => 1; "case 2")]
    #[test_case(vec![0, 1] => 2; "case 3")]
    #[test_case(vec![1,0,3,4,3] => 3; "case 4")]
    #[test_case(vec![6,6,6,6,6,6] => 6; "case 5")]
    #[test_case(vec![1,1,6,5,6,6,1,1,1,1] => 6; "case 6")]
    fn test_total_fruit(fruits: Vec<i32>) -> i32 {
        Solution::total_fruit(fruits)
    }
}
