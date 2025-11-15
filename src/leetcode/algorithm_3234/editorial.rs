pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            if i == 0 || s[i - 1] == b'0' {
                prefix[i + 1] = i;
                continue;
            }
            prefix[i + 1] = prefix[i];
        }
        let mut count = 0;
        for (i, &c) in s.iter().enumerate() {
            let mut zeros: usize = if c == b'0' { 1 } else { 0 };
            let mut j = i + 1;
            while j > 0 && zeros.pow(2) <= n {
                let zeros_next = i + 1 - prefix[j] - zeros;
                if zeros.pow(2) <= zeros_next {
                    count += (j - prefix[j]).min(zeros_next - zeros.pow(2) + 1)
                }
                j = prefix[j];
                zeros += 1;
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("00011" => 5; "example 1")]
    #[test_case("101101" => 16; "example 2")]
    fn test_solution(s: &str) -> i32 {
        Solution::number_of_substrings(String::from(s))
    }
}
