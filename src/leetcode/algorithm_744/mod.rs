pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let first = letters[0];
        letters.into_iter().find(|x| *x > target).unwrap_or(first)
    }
}
