use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let m = languages.len();
        let mut connected = Vec::from_iter(0..m);
        let mut language_count: Vec<BTreeMap<i32, usize>> = vec![BTreeMap::new(); m];
        for (i, language) in languages.iter().enumerate() {
            for &l in language {
                *language_count[i].entry(l).or_default() += 1;
            }
        }
        fn find_parent(connected: &mut [usize], u: usize) -> usize {
            let mut parent = u;
            let mut to_update = vec![];
            while connected[parent] != parent {
                parent = connected[parent];
                to_update.push(parent);
            }
            for p in to_update.into_iter() {
                connected[p] = parent;
            }
            parent
        }
        for friendship in friendships.iter() {
            let a = (friendship[0] - 1) as usize;
            let b = (friendship[1] - 1) as usize;
            let a_parent = find_parent(&mut connected, a);
            let b_parent = find_parent(&mut connected, b);
            let new_root = a_parent.min(b_parent);
            connected[a_parent] = new_root;
            connected[b_parent] = new_root;
            language_count[new_root] =
                Self::merge_count(&language_count[a_parent], &language_count[b_parent]);
        }
        let common_languages: Vec<i32> = language_count
            .iter()
            .map(|counts| {
                let mut i = 0;
                let mut count = 0;
                for (k, &v) in counts {
                    if v > count {
                        count = v;
                        i = *k;
                    }
                }
                i
            })
            .collect();
        let mut count = 0;
        for u in 0..m {
            let language = common_languages[find_parent(&mut connected, u)];
            if languages[u].contains(&language) {
                continue;
            }
            count += 1;
        }
        count
    }

    fn merge_count(a: &BTreeMap<i32, usize>, b: &BTreeMap<i32, usize>) -> BTreeMap<i32, usize> {
        let mut merged = a.clone();
        for (&k, &v) in b.iter() {
            *merged.entry(k).or_default() += v;
        }
        merged
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
    fn test_solution(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        Solution::minimum_teachings(n, languages, friendships)
    }
}
