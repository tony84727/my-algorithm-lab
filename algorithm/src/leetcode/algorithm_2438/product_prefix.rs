pub struct Solution;

impl Solution {
    pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let modulo = 10_u64.pow(9) + 7;
        let mut bits: Vec<u32> = vec![];
        let mut digit = 0;
        while n != 0 {
            if n % 2 != 0 {
                bits.push(digit);
            }
            digit += 1;
            n /= 2;
        }
        let mut precalculate: Vec<u64> = vec![1; bits.len() + 1];
        precalculate[0] = 1;
        for (i, b) in bits.into_iter().enumerate() {
            precalculate[i + 1] = precalculate[i] * 2_u64.pow(b);
        }
        queries
            .into_iter()
            .map(|query| {
                let left = query[0];
                let right = query[1];
                ((precalculate[right as usize + 1] / precalculate[left as usize]) % modulo) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vecvec;
    use test_case::test_case;

    #[test_case(15, vecvec![[0,1],[2,2], [0,3]] => vec![2,4,64]; "example 1")]
    #[test_case(2, vec![vec![0,0]] => vec![2]; "example 2")]
    #[test_case(919, vecvec![[5,5],[4,4],[0,1],[1,5],[4,6],[6,6],[5,6],[0,3],[5,5],[5,6],[1,2],[3,5],[3,6],[5,5],[4,4],[1,1],[2,4],[4,5],[4,4],[5,6],[0,4],[3,3],[0,4],[0,5],[4,4],[5,5],[4,6],[4,5],[0,4],[6,6],[6,6],[6,6],[2,2],[0,5],[1,4],[0,3],[2,4],[5,5],[6,6],[2,2],[2,3],[5,5],[0,6],[3,3],[6,6],[4,4],[0,0],[0,2],[6,6],[6,6],[3,6],[0,4],[6,6],[2,2],[4,6]] => vec![256,128,2,4194304,16777216,512,131072,128,256,131072,8,524288,268435456,256,128,2,8192,32768,128,131072,16384,16,16384,4194304,128,256,16777216,32768,16384,512,512,512,4,4194304,16384,128,8192,256,512,4,64,256,147483634,16,512,128,1,8,512,512,268435456,16384,512,4,16777216]; "case 1")]
    //#[test_case(806335498, vec![vec![0,0]] => vec![1]; "case 2")]
    fn test_solution(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Solution::product_queries(n, queries)
    }
}
