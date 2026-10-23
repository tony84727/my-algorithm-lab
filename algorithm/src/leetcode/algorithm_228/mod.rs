pub struct Solution;
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut start = None;
        let mut current = 0;
        let mut ranges = vec![];
        for n in nums.into_iter() {
            match start {
                None => {
                    start = Some(n);
                    current = n;
                }
                Some(from) => {
                    if n != current + 1 {
                        ranges.push(if current == from {
                            from.to_string()
                        } else {
                            format!("{from}->{current}")
                        });
                        start = Some(n);
                        current = n;
                        continue;
                    }
                    current += 1;
                }
            }
        }
        if let Some(start) = start {
            ranges.push(if current == start {
                start.to_string()
            } else {
                format!("{start}->{current}")
            });
        }
        ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,1,2,4,5,7] => vec!["0->2".to_string(), "4->5".to_string(),"7".to_string()]; "example 1")]
    #[test_case(vec![0,2,3,4,6,8,9] => vec![
        "0".to_string(),
        "2->4".to_string(),
        "6".to_string(),
        "8->9".to_string(),
    ]; "example 2")]
    fn test_solution(nums: Vec<i32>) -> Vec<String> {
        Solution::summary_ranges(nums)
    }
}
