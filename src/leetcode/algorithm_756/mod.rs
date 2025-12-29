pub struct Solution;

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut map = vec![vec![vec![]; 6]; 6];
        for pair in allowed.into_iter() {
            let pair = pair.as_bytes();
            map[Self::index(pair[0])][Self::index(pair[1])].push(pair[2]);
        }
        let current = bottom.as_bytes().to_vec();
        let mut next = vec![];
        Self::dfs(&current, &mut next, &map)
    }

    fn dfs(current: &Vec<u8>, next: &mut Vec<u8>, map: &[Vec<Vec<u8>>]) -> bool {
        if current.len() == 1 {
            return true;
        }
        if next.len() == current.len() - 1 {
            return Self::dfs(next, &mut vec![], map);
        }
        let left = current[next.len()];
        let right = current[next.len() + 1];
        let candidates = &map[Self::index(left)][Self::index(right)];
        for &block in candidates.iter() {
            next.push(block);
            if Self::dfs(current, next, map) {
                return true;
            }
            next.pop();
        }
        false
    }

    fn index(i: u8) -> usize {
        (i - b'A') as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("BCD", &["BCC","CDE","CEA","FFF"] => true; "example 1")]
    #[test_case("AAAA", &["AAB","AAC","BCD","BBE","DEF"] => false; "example 2")]
    fn test_solution(bottom: &str, allowed: &[&str]) -> bool {
        Solution::pyramid_transition(
            String::from(bottom),
            allowed.iter().map(|x| x.to_string()).collect(),
        )
    }
}
