pub struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let chars = target.chars().collect::<Vec<char>>();
        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();
        Self::ways(&words, 0, &chars)
    }

    fn ways(words: &Vec<Vec<char>>, i: usize, target: &[char]) -> i32 {
        if target.is_empty() {
            return 1;
        }
        let head = target[0];
        let mut ways = 0;
        for w in words.iter() {
            for (i, c) in w.iter().enumerate().skip(i) {
                if *c == head {
                    ways += Self::ways(words, i + 1, &target[1..]);
                }
            }
        }
        ways
    }
}
