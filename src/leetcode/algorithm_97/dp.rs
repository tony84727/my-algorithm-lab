pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.is_empty() && s2.is_empty() && s3.is_empty() {
            return true;
        }
        fn solve(
            memoized: &mut Vec<Vec<Vec<Option<bool>>>>,
            s1: &str,
            s2: &str,
            s3: &str,
            begin_by_s2: bool,
            i1: usize,
            i2: usize,
        ) -> bool {
            if i1 > s1.len() || i2 > s2.len() {
                return false;
            }
            if i1 == s1.len() && i2 == s2.len() && i1 + i2 == s3.len() {
                return true;
            }
            let path_index = if begin_by_s2 { 0 } else { 1 };
            if let Some(answer) = memoized
                .get(path_index)
                .and_then(|x| x.get(i1))
                .and_then(|x| x.get(i2))
                .and_then(|x| *x)
            {
                return answer;
            }
            let s = if begin_by_s2 { &s2 } else { &s1 };
            let start = if begin_by_s2 { i2 } else { i1 };
            for (i, (j, k)) in s
                .chars()
                .skip(start)
                .zip(s3.chars().skip(i1 + i2))
                .enumerate()
            {
                if j != k {
                    break;
                }
                if solve(
                    memoized,
                    s1,
                    s2,
                    s3,
                    !begin_by_s2,
                    if begin_by_s2 { i1 } else { i1 + i + 1 },
                    if begin_by_s2 { i2 + i + 1 } else { i2 },
                ) {
                    memoized[path_index][i1][i2] = Some(true);
                    return true;
                }
            }
            memoized[path_index][i1][i2] = Some(false);
            false
        }
        let mut memoized = vec![vec![vec![None; s2.len() + 1]; s1.len() + 1]; 2];
        solve(&mut memoized, &s1, &s2, &s3, false, 0, 0)
            || solve(&mut memoized, &s1, &s2, &s3, true, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aabcc", "dbbca", "aadbbcbcac" => true; "example 1")]
    #[test_case("aabcc", "dbbca", "aadbbbaccc" => false; "example 2")]
    #[test_case("", "", "" => true; "example 3")]
    #[test_case("a", "", "a" => true; "case 1")]
    #[test_case("", "", "a" => false; "case 2")]
    #[test_case("a", "b", "a" => false; "case 3")]
    fn test_solution(s1: &'static str, s2: &'static str, s3: &'static str) -> bool {
        Solution::is_interleave(String::from(s1), String::from(s2), String::from(s3))
    }
}
