pub struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut out_roads = vec![vec![]; n as usize];
        for (i, r) in roads.iter().enumerate() {
            out_roads[(r[0] - 1) as usize].push(i);
            out_roads[(r[1] - 1) as usize].push(i);
        }
        let mut visited = vec![false; n as usize];
        let mut to_visit: Vec<i32> = vec![1];
        let mut min = i32::MAX;
        while let Some(city) = to_visit.pop() {
            let id = (city - 1) as usize;
            visited[id] = true;
            for &r in out_roads[id].iter() {
                let a = roads[r][0];
                let b = roads[r][1];
                let cost = roads[r][2];
                min = min.min(cost);
                if !visited[(a - 1) as usize] {
                    to_visit.push(a);
                }
                if !visited[(b - 1) as usize] {
                    to_visit.push(b);
                }
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(14, vec![
            vec![2,9,2308],
            vec![2,5,2150],
            vec![12,3,4944],
            vec![13,5,5462],
            vec![2,10,2187],
            vec![2,12,8132],
            vec![2,13,3666],
            vec![4,14,3019],
            vec![2,4,6759],
            vec![2,14,9869],
            vec![1,10,8147],
            vec![3,4,7971],
            vec![9,13,8026],
            vec![5,12,9982],
            vec![10,9,6459],
    ] => 2150; "case 1")]
    #[test_case(9, vec![
            vec![6,2,7],
            vec![3,7,2],
            vec![9,6,5],
            vec![2,4,9],
            vec![7,8,7],
            vec![8,4,5],
            vec![6,1,10],
        ] => 2; "case 2")]
    fn test_solution(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        Solution::min_score(n, roads)
    }
}
