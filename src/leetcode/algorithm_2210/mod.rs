pub struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut wait_increasing: Option<bool> = None;
        let mut count = 0;
        if nums.is_empty() {
            return 0;
        }
        let mut last: Option<i32> = None;
        for n in nums.into_iter() {
            match last {
                Some(previous) => {
                    if n == previous {
                        continue;
                    }
                    if n > previous {
                        if let Some(true) = wait_increasing {
                            count += 1;
                        }
                        wait_increasing = Some(false);
                    }
                    if n < previous {
                        if let Some(false) = wait_increasing {
                            count += 1;
                        }
                        wait_increasing = Some(true);
                    }
                    last = Some(n);
                }
                None => {
                    last = Some(n);
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,4,1,1,6,5] => 3; "example 1")]
    #[test_case(vec![6,6,5,5,4,1] => 0; "example 2")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::count_hill_valley(nums)
    }
}
