pub struct Solution;

impl Solution {
    fn index(c: char) -> usize {
        c as usize - 'a' as usize
    }
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return Vec::new();
        }
        let mut p_counts = vec![0; 26];
        let mut s_counts = vec![0; 26];
        for (p, s) in p.chars().map(Self::index).zip(s.chars().map(Self::index)) {
            p_counts[p] += 1;
            s_counts[s] += 1;
        }
        let mut found = vec![];
        let chars = s.chars().collect::<Vec<char>>();
        if p_counts == s_counts {
            found.push(0_i32);
        }
        for (i, c) in chars.iter().enumerate().skip(p.len()) {
            s_counts[Self::index(*c)] += 1;
            s_counts[Self::index(chars[i - p.len()])] -= 1;
            if p_counts == s_counts {
                found.push((i - p.len() + 1) as i32);
            }
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("cbaebabacd", "abc" => vec![0,6]; "example 1")]
    #[test_case("abab", "ab" => vec![0,1,2]; "example 2")]
    fn test_solution(s: &str, p: &str) -> Vec<i32> {
        Solution::find_anagrams(s.to_owned(), p.to_owned())
    }
}
