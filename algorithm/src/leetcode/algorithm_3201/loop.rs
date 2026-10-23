pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut all_odd_count = 0;
        let mut all_even_count = 0;
        let mut odd_count = 0;
        let mut even_count = 0;
        let mut old_start_next_odd = true;
        let mut even_start_next_odd = false;
        for n in nums.iter() {
            let odd = n % 2 == 1;
            if odd {
                all_odd_count += 1;
            } else {
                all_even_count += 1;
            }
            if old_start_next_odd && odd {
                odd_count += 1;
                old_start_next_odd = !old_start_next_odd;
            }
            if !old_start_next_odd && !odd {
                odd_count += 1;
                old_start_next_odd = !old_start_next_odd;
            }
            if even_start_next_odd && odd {
                even_count += 1;
                even_start_next_odd = !even_start_next_odd;
            }
            if !even_start_next_odd && !odd {
                even_count += 1;
                even_start_next_odd = !even_start_next_odd;
            }
        }
        all_odd_count
            .max(odd_count)
            .max(even_count)
            .max(all_even_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4] => 4; "example 1")]
    #[test_case(vec![1,2,1,1,2,1,2] => 6; "example 2")]
    #[test_case(vec![5,3,7] => 3; "case 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::maximum_length(nums)
    }
}
