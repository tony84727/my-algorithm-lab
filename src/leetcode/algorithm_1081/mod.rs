pub struct Solution;

fn index(x: char) -> usize {
    (x as u8 - b'a') as usize
}

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut last_index = [None; 26];
        for (i, c) in s.char_indices() {
            last_index[index(c)] = Some(i);
        }
        let mut used = [false; 26];
        let mut answer: Vec<char> = vec![];
        for (i, c) in s.char_indices() {
            let ci = index(c);
            if used[ci] {
                continue;
            }
            while let Some(top) = answer.last().cloned() {
                let ti = index(top);
                if top > c && last_index[ti] > Some(i) {
                    answer.pop();
                    used[ti] = false;
                } else {
                    break;
                }
            }
            used[ci] = true;
            answer.push(c);
        }
        answer.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(String::from("bcabc") => String::from("abc"))]
    #[test_case(String::from("cbacdcbc") => String::from("acdb"))]
    fn test_solution(s: String) -> String {
        Solution::smallest_subsequence(s)
    }
}
