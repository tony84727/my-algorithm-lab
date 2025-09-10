use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut need_learn = HashSet::new();
        for f in friendships.iter() {
            let a = (f[0] - 1) as usize;
            let b = (f[1] - 1) as usize;
            let mut counts = vec![0; n as usize];
            let mut overlap = false;
            for &l in languages[a].iter() {
                counts[l as usize - 1] += 1;
            }
            for &l in languages[b].iter() {
                if counts[l as usize - 1] > 0 {
                    overlap = true;
                    break;
                }
                counts[l as usize - 1] += 1;
            }
            if !overlap {
                need_learn.insert(a);
                need_learn.insert(b);
            }
        }
        let mut common_languages = vec![0; n as usize];
        let mut max = 0;
        for f in need_learn.iter() {
            for l in languages[*f].iter() {
                common_languages[*l as usize - 1] += 1;
                max = common_languages[*l as usize - 1].max(max)
            }
        }
        (need_learn.len() - max) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(2, vecvec![[1],[2],[1,2]], vecvec![[1,2],[1,3],[2,3]] => 1; "example 1")]
    #[test_case(3, vecvec![[2],[1,3],[1,2],[3]], vecvec![[1,4],[1,2],[3,4],[2,3]] => 2; "example 2")]
    #[test_case(11, vecvec![[3,11,5,10,1,4,9,7,2,8,6],[5,10,6,4,8,7],[6,11,7,9],[11,10,4],[6,2,8,4,3],[9,2,8,4,6,1,5,7,3,10],[7,5,11,1,3,4],[3,4,11,10,6,2,1,7,5,8,9],[8,6,10,2,3,1,11,5],[5,11,6,4,2]], vecvec![[7,9],[3,7],[3,4],[2,9],[1,8],[5,9],[8,9],[6,9],[3,5],[4,5],[4,9],[3,6],[1,7],[1,3],[2,8],[2,6],[5,7],[4,6],[5,8],[5,6],[2,7],[4,8],[3,8],[6,8],[2,5],[1,4],[1,9],[1,6],[6,7]] => 0; "case 1")]
    #[test_case(4, vecvec![[1,2],[2],[3],[3],[4]], vecvec![[1,2], [3,4]] => 0; "case 2")]
    #[test_case(4, vecvec![[1,3], [1,2],[3,4]], vecvec![[1,2], [1,3]] => 0; "case 3")]
    fn test_solution(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        Solution::minimum_teachings(n, languages, friendships)
    }
}
