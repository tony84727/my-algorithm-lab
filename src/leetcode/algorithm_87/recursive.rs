pub struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::sub(&s1, &s2)
    }

    fn sub(s1: &str, s2: &str) -> bool {
        if s1.len() <= 1 {
            return s1 == s2;
        }
        for i in 1..s1.len() {
            let (s1a, s1b) = s1.split_at(i);
            let (s2a, s2b) = s2.split_at(i);
            let (s2bs, s2as) = s2.split_at(s1.len() - i);
            if Self::sub(s1a, s2a) && Self::sub(s1b, s2b)
                || (Self::sub(s1a, s2as) && Self::sub(s1b, s2bs))
            {
                return true;
            }
        }
        false
    }
}
