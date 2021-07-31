pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        strs.sort_by_key(|s| s.len());
        let mut i = 0;
        let shortest = &strs[0];
        'search: while i < shortest.len() {
            let c = strs[0].chars().nth(i).unwrap();
            for str in strs.iter().skip(1) {
                let s = str.chars().nth(i).unwrap();
                if c != s {
                    break 'search;
                }
            }
            i += 1;
        }
        shortest[0..i].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["flower","flow","flight"] => "fl"; "example 1")]
    fn test_solution(strs: &[&str]) -> String {
        Solution::longest_common_prefix(strs.iter().map(|s| s.to_string()).collect())
    }
}
