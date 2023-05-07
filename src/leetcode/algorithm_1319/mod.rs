pub struct Solution;
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if (n - 1) as usize > connections.len() {
            return -1;
        }
        let mut connected = (0..n).collect::<Vec<i32>>();
        for connection in connections.into_iter() {
            let a = connection[0] as usize;
            let b = connection[1] as usize;
            if connected[a] == connected[b] {
                continue;
            }
            let mut endpoints = [connected[a] as usize, connected[b] as usize];
            endpoints.sort_unstable();
            let [to, from] = endpoints;
            for n in connected.iter_mut() {
                if *n == from as i32 {
                    *n = to as i32;
                }
            }
        }
        connected.sort_unstable();
        connected.dedup();
        (connected.len() - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(4, vec![vec![0,1],vec![0,2],vec![1,2]] => 1; "example 1")]
    #[test_case(6,vec![vec![0,1],vec![0,2],vec![0,3],vec![1,2],vec![1,3]] => 2; "example 2")]
    #[test_case(6, vec![vec![0,1],vec![0,2], vec![0,3],vec![1,2]] => -1; "example 3")]
    fn test_solution(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        Solution::make_connected(n, connections)
    }
}
