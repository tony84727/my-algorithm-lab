pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        for segment in path.split("/") {
            match segment {
                ".." => {
                    stack.pop();
                }
                x if x.is_empty() => (),
                "." => (),
                _ => stack.push(segment),
            }
        }
        "/".to_owned() + &stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("/home/".to_owned() => "/home".to_owned(); "example 1")]
    #[test_case("/../".to_owned() => "/".to_owned(); "example 2")]
    #[test_case("/a/./b/../../c/".to_owned() => "/c".to_owned(); "case 1")]
    fn test_solution(path: String) -> String {
        Solution::simplify_path(path)
    }
}
