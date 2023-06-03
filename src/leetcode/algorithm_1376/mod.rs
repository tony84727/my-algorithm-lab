pub struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let head_id = head_id as usize;
        let n = n as usize;
        let mut children = vec![vec![]; n];
        for (i, manager) in manager.iter().enumerate() {
            if i == head_id {
                continue;
            }
            children[*manager as usize].push(i);
        }
        fn dfs(children: &[Vec<usize>], time: &[i32], current: usize) -> i32 {
            children[current]
                .iter()
                .map(|next| dfs(children, time, *next))
                .max()
                .unwrap_or_default()
                + time[current]
        }
        dfs(&children, &inform_time, head_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 0, vec![-1], vec![0] => 0; "example 1")]
    #[test_case(6,2, vec![2,2,-1,2,2,2], vec![0,0,1,0,0,0] => 1; "example 2")]
    fn test_solution(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        Solution::num_of_minutes(n, head_id, manager, inform_time)
    }
}
