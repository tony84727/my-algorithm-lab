pub struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a, b| {
            let ao = a.count_ones();
            let bo = b.count_ones();
            ao.cmp(&bo).then_with(|| a.cmp(b))
        });
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,1,2,3,4,5,6,7,8] => vec![0,1,2,4,8,3,5,6,7]; "example 1")]
    #[test_case(vec![1024,512,256,128,64,32,16,8,4,2,1] => vec![1,2,4,8,16,32,64,128,256,512,1024]; "example 2")]
    fn test_solution(arr: Vec<i32>) -> Vec<i32> {
        Solution::sort_by_bits(arr)
    }
}
