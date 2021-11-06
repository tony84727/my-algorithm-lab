pub struct Solution;

impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut counter = 0;
        let mut last = None;
        for n in nums.into_iter() {
            match last {
                Some(before) => {
                    if before == n {
                        counter += 1;
                        continue;
                    }
                    if counter == 1 {
                        return before;
                    }
                    counter = 1;
                    last = Some(n);
                }
                None => {
                    last = Some(n);
                    counter = 1;
                }
            }
        }
        if let Some(last) = last {
            if counter == 1 {
                return last;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,2,1] => 1; "example 1")]
    #[test_case(vec![4,1,2,1,2] => 4; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::single_number(nums)
    }
}
