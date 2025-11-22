pub struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut left_index: Vec<Option<usize>> = vec![None; 26];
        let mut right_index: Vec<Option<usize>> = vec![None; 26];
        let mut unique_middle: Vec<Vec<usize>> = vec![Vec::new(); 26];
        for (i, c) in s.char_indices() {
            let index = c as usize - 'a' as usize;
            unique_middle[index].push(i);
            if left_index[index].is_none() {
                left_index[index] = Some(i);
            }
            if let Some(right) = right_index[index] {
                if right < i {
                    right_index[index] = Some(i);
                }
            } else {
                right_index[index] = Some(i);
            }
        }
        let mut count = 0;
        for side in 0..26 {
            let Some(l) = left_index[side] else {
                continue;
            };
            let Some(r) = right_index[side] else {
                continue;
            };
            if r <= l {
                continue;
            }
            for index_list in unique_middle.iter() {
                for &i in index_list.iter() {
                    if i > l && i < r {
                        count += 1;
                        break;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aabca" => 3; "example 1")]
    #[test_case("adc" => 0; "example 2")]
    #[test_case("bbcbaba" => 4; "example 3")]
    fn test_solution(s: &str) -> i32 {
        Solution::count_palindromic_subsequence(String::from(s))
    }
}
