pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let ones = nums.iter().filter(|&&x| x == 1).count();
        if ones > 0 {
            return (n - ones) as i32;
        }

        let mut best = i32::MAX;
        for i in 0..n {
            let mut g = nums[i];
            for j in i..n {
                g = Self::gcd(g, nums[j]);
                if g == 1 {
                    best = best.min((j - i + 1) as i32);
                    break;
                }
            }
        }

        if best == i32::MAX {
            -1
        } else {
            best + n as i32 - 2
        }
    }

    fn gcd(x: i32, y: i32) -> i32 {
        let mut a = x;
        let mut b = y;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,6,3,4] => 4; "example 1")]
    #[test_case(vec![2,10,6,14] => -1; "example 2")]
    #[test_case(vec![1,2,2,4,4,5,6,1,2,3,10] => 9; "case 1")]
    #[test_case(vec![3,3,3] => -1; "no way")]
    #[test_case(vec![6,10,15] => 4; "need to create first one")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::min_operations(nums)
    }
}
