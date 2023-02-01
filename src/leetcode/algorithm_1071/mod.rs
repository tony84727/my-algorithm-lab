pub struct Solution;

impl Solution {
    pub fn gcd_of_strings(mut str1: String, mut str2: String) -> String {
        while str1.len() != 0 && str2.len() != 0 {
            if str1.len() > str2.len() {
                let (effective, result) = Self::trim(str1, &str2);
                if !effective {
                    return String::new();
                }
                str1 = result;
            } else {
                let (effective, result) = Self::trim(str2, &str1);
                if !effective {
                    return String::new();
                }
                str2 = result;
            }
        }
        str1 + &str2
    }

    fn trim(a: String, b: &str) -> (bool, String) {
        let len = a.len();
        let start = len - b.len();
        if &a[start..] == b {
            return (true, a[..start].to_owned());
        }
        (false, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ABCABC", "ABC" => "ABC"; "example 1")]
    #[test_case("ABABAB", "ABAB" => "AB"; "example 2")]
    #[test_case("LEET", "CODE" => ""; "example 3")]
    #[test_case("AABB", "AB" => ""; "case 1")]
    fn test_solution(str1: &str, str2: &str) -> String {
        Solution::gcd_of_strings(str1.to_owned(), str2.to_owned())
    }
}
