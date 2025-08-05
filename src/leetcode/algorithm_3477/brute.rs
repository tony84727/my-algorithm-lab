pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut unplaced = 0;
        for f in fruits.into_iter() {
            let mut placed: Option<usize> = None;
            for (i, &b) in baskets.iter().enumerate() {
                if f <= b {
                    placed = Some(i);
                    break;
                }
            }
            if let Some(placed) = placed {
                baskets.remove(placed);
                continue;
            }
            unplaced += 1;
        }
        unplaced
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,2,5], vec![3,5,4] => 1; "example 1")]
    #[test_case(vec![3,6,1], vec![6,4,7] => 0; "example 2")]
    fn test_solution(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        Solution::num_of_unplaced_fruits(fruits, baskets)
    }
}
