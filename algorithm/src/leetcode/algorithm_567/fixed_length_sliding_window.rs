pub struct Solution;

impl Solution {
    fn index(c: char) -> usize {
        c as usize - 'a' as usize
    }
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let mut s1_counts = vec![0; 26];
        let mut s2_counts = vec![0; 26];
        let chars = s2.chars().collect::<Vec<char>>();
        for (i, s1_index) in s1.chars().map(Self::index).enumerate() {
            s1_counts[s1_index] += 1;
            s2_counts[Self::index(chars[i])] += 1;
        }
        if s1_counts == s2_counts {
            return true;
        }
        for i in s1.len()..s2.len() {
            s2_counts[Self::index(chars[i])] += 1;
            s2_counts[Self::index(chars[i - s1.len()])] -= 1;
            if s1_counts == s2_counts {
                return true;
            }
        }
        false
    }
}
