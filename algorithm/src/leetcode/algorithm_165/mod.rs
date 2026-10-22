use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = version1.split(".");
        let mut v2 = version2.split(".");
        loop {
            let v1 = v1.next().and_then(|x| x.parse::<i32>().ok());
            let v2 = v2.next().and_then(|x| x.parse::<i32>().ok());
            if v1.is_none() && v2.is_none() {
                return 0;
            }
            let cmp = v1.unwrap_or_default().cmp(&v2.unwrap_or_default());
            if cmp != Ordering::Equal {
                return match cmp {
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1.2", "1.10" => -1; "example 1")]
    #[test_case("1.01", "1.001" => 0; "example 2")]
    #[test_case("1.0", "1.0.0.0" => 0; "example 3")]
    fn test_solution(version1: &str, version2: &str) -> i32 {
        Solution::compare_version(String::from(version1), String::from(version2))
    }
}
