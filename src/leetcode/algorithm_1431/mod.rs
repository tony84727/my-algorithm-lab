pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let maximum = *match candies.iter().max() {
            Some(m) => m,
            None => return vec![],
        };
        candies
            .into_iter()
            .map(|x| x + extra_candies >= maximum)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,3,5,1,3], 3 => vec![true,true,true,false,true];"example 1")]
    fn test_solution(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        Solution::kids_with_candies(candies, extra_candies)
    }
}
