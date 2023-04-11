pub struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '*' => {
                    stack.pop();
                }
                _ => stack.push(c),
            }
        }
        stack.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("leet**cod*e" => "lecoe".to_owned();"example 1")]
    #[test_case("erase*****" => "".to_owned(); "example 2")]
    fn test_solution(s: &str) -> String {
        Solution::remove_stars(s.to_owned())
    }
}
