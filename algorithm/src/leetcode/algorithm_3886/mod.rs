pub struct Solution;

fn check(k: usize, nums: &[i32]) -> bool {
    let mut last_max: Option<i32> = None;

    for sub in nums.chunks(k) {
        let mut min = sub[0];
        let mut max = sub[0];
        let mut down = false;
        for (i, x) in sub.iter().cloned().enumerate().skip(1) {
            min = min.min(x);
            max = max.max(x);
            if sub[i - 1] > x {
                if down {
                    return false;
                }
                down = true;
            }
        }
        if down && sub[k - 1] > sub[0] {
            return false;
        }
        if let Some(m) = last_max
            && m > min
        {
            return false;
        }
        last_max = Some(max);
    }
    true
}

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        for k in 1..=n {
            if !n.is_multiple_of(k) {
                continue;
            }
            if check(k, &nums) {
                result += k;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3, 1, 2] => 3; "example 1")]
    #[test_case(vec![7, 6, 5] => 0; "example 2")]
    #[test_case(vec![5, 8] => 3; "example 3")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::sortable_integers(nums)
    }
}
