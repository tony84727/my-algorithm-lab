pub struct Solution;

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let k = k as usize;
        let modulo: i64 = 1000000007;
        let mut repeated: Vec<usize> = vec![];
        let mut last: Option<char> = None;
        for k in word.chars() {
            if Some(k) == last {
                *repeated.last_mut().unwrap() += 1;
                continue;
            }
            last = Some(k);
            repeated.push(1);
        }
        let total = repeated.iter().fold(1, |a, b| (a * b) % modulo as usize);
        if repeated.len() >= k {
            return total as i32;
        }
        let mut ways: Vec<Vec<i64>> = vec![vec![0; k]; repeated.len() + 1];
        ways[0][0] = 1;
        for (i, f) in repeated.iter().enumerate() {
            for j in 0..k {
                for uses in 1..=*f {
                    if j >= uses {
                        ways[i + 1][j] += ways[i][j - uses];
                        ways[i + 1][j] %= modulo;
                    }
                }
            }
        }
        ((total as i64 - ways[repeated.len()].iter().fold(0, |a, b| (a + b) % modulo) + modulo)
            % modulo) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aabbccdd", 7 => 5; "example 1")]
    #[test_case("aabbccdd", 8 => 1; "example 2")]
    #[test_case("aaabbb", 3 => 8; "example 3")]
    #[test_case("abbc", 3 => 2; "case 1")]
    #[test_case("dd", 1 => 2; "case 2")]
    #[test_case("bbbbbyyyyyyyyyyccccccccyyyqqqqhffffhhhhhhhhsswwwwvvvvvlllldddddddddnnnnnnvr", 69 => 23761; "case 3")]
    #[test_case("mmzzzzzbbbbbbbbbmmyyyyyyyyttttttzzzooogggggggggyyyyyyyyhhhttllllhhhhqcccchh", 65 => 2121993; "case 4")]
    #[test_case("ggggggggaaaaallsssssaaaaaaaaaiiqqqqqqqqqqbbbbbbbvvfffffjjjjeeeeeefffmmiiiix", 34 => 834168507; "case 5")]
    fn test_solution(word: &'static str, k: i32) -> i32 {
        Solution::possible_string_count(String::from(word), k)
    }
}
