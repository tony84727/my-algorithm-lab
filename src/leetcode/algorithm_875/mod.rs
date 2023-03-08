pub struct Solution;

impl Solution {
    pub fn test(piles: &[i32], n: i32, mut h: i32) -> bool {
        for b in piles.iter() {
            h -= (*b as f64 / n as f64).ceil() as i32;
            if h < 0 {
                return false;
            }
        }
        h >= 0
    }
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut max = i32::MAX;
        let mut min = 0;
        let mut last = max;
        while min <= max {
            let mid = min + (max - min) / 2;
            if Self::test(&piles, mid, h) {
                max = mid - 1;
                last = mid;
            } else {
                min = mid + 1;
            }
        }
        last
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![3,6,7,11], 8 => 4; "example 1")]
    #[test_case(vec![332484035,524908576,855865114,632922376,222257295,690155293,112677673,679580077,337406589,290818316,877337160,901728858,679284947,688210097,692137887,718203285,629455728,941802184], 823855818 => 14; "case 1")]
    #[test_case(vec![1000000000], 2 => 500000000; "case 2")]
    fn test_solution(piles: Vec<i32>, h: i32) -> i32 {
        Solution::min_eating_speed(piles, h)
    }
}
