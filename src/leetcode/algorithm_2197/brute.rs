pub struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        Self::solve(&nums)
    }

    fn solve(nums: &[i32]) -> Vec<i32> {
        if nums.len() <= 1 {
            return Vec::from(nums);
        }
        if nums.len() == 2 {
            let gcd = Self::gcd(nums[0], nums[1]);
            if gcd > 1 {
                return vec![(nums[0] as i64 * nums[1] as i64 / gcd as i64) as i32];
            } else {
                return Vec::from(nums);
            }
        }
        let mut mid = nums.len() / 2;
        let left = Self::solve(&nums[..mid]);
        let o_right = Self::solve(&nums[mid..]);
        let mut merged = left.clone();
        let mut right = o_right.clone();
        mid = merged.len();
        merged.append(&mut right);
        if merged.len() < 2 {
            return merged;
        }
        let mut intersection = Self::solve(&merged[mid - 1..=mid]);
        if intersection.len() == 1 {
            let mut reduced = merged[..mid - 1].to_vec();
            reduced.append(&mut intersection);
            reduced.append(&mut merged[mid + 1..].to_vec());
            return Self::solve(&reduced);
        }
        merged
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 && b != 0 {
            (a, b) = (a.max(b), a.min(b));
            a -= b;
        }
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(6,3 => 3)]
    #[test_case(10,4 => 2)]
    fn test_gcd(a: i32, b: i32) -> i32 {
        Solution::gcd(a, b)
    }

    #[test_case(vec![6,4,3,2,7,6,2] => vec![12,7,6]; "example 1")]
    #[test_case(vec![2,2,1,1,3,3,3] => vec![2,1,1,3]; "example 2")]
    #[test_case(vec![517,11,121,517,3,51,3,1887,5] => vec![5687,1887,5]; "case 1")]
    #[test_case(vec![5687,3,1887,5] => vec![5687,1887,5]; "case 2")]
    #[test_case(vec![31,97561,97561,97561,97561,97561,97561,97561,97561] => vec![31,97561]; "case 3")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        Solution::replace_non_coprimes(nums)
    }
}
