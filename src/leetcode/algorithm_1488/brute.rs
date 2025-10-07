use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut answers = Vec::with_capacity(rains.len());
        let mut filled: HashMap<i32, usize> = HashMap::new();
        'day: for (d, r) in rains.into_iter().enumerate() {
            if r > 0 {
                answers.push(-1);
                if let Some(filled_at) = filled.get(&r) {
                    for a in &mut answers[(*filled_at + 1)..d] {
                        if *a == -2 {
                            *a = r;
                            filled.insert(r, d);
                            continue 'day;
                        }
                    }
                    return Vec::new();
                }
                filled.insert(r, d);
                continue;
            }
            answers.push(-2);
        }
        answers
            .into_iter()
            .map(|x| if x == -2 { 1 } else { x })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4] => vec![-1,-1,-1,-1]; "example 1")]
    #[test_case(vec![1,2,0,0,2,1] => vec![-1,-1,2,1,-1,-1]; "example 2")]
    #[test_case(vec![1,2,0,1,2] => Vec::<i32>::new(); "example 3")]
    #[test_case(vec![0,1,1] => Vec::<i32>::new(); "case 1")]
    fn test_solution(rains: Vec<i32>) -> Vec<i32> {
        Solution::avoid_flood(rains)
    }
}
