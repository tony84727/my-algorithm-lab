pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut a = word1.chars();
        let mut b = word2.chars();
        let mut x = a.next();
        let mut y = b.next();
        let mut answer = String::new();
        loop {
            match (x, y) {
                (None, None) => break,
                (Some(a), None) => {
                    answer.push(a);
                }
                (None, Some(b)) => {
                    answer.push(b);
                }
                (Some(a), Some(b)) => {
                    answer.push(a);
                    answer.push(b);
                }
            }
            x = a.next();
            y = b.next();
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abc","pqr" => "apbqcr".to_owned(); "example 1")]
    #[test_case("ab","pqrs" => "apbqrs".to_owned(); "example 2")]
    fn test_solution(word1: &str, word2: &str) -> String {
        Solution::merge_alternately(word1.to_owned(), word2.to_owned())
    }
}
