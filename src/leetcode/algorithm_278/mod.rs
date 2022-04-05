pub struct Solution {
    bad_version_after: i32,
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.is_bad_version(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }

    fn is_bad_version(&self, n: i32) -> bool {
        n >= self.bad_version_after
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5,4; "example 1")]
    #[test_case(2126753390, 1702766719)]
    fn test_solution(n: i32, bad_version_after: i32) {
        assert_eq!(
            bad_version_after,
            Solution { bad_version_after }.first_bad_version(n)
        );
    }
}
