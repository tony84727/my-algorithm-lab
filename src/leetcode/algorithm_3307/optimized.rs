pub struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut current = k as usize - 1;
        let mut shift = 0;
        while current != 0 {
            let batch = (current as f32).log2().floor() as u32;
            let jump = 2_usize.pow(batch);
            if operations[batch as usize] == 1 && current >= 2_usize.pow(batch) {
                shift += 1;
            }
            current -= jump;
        }
        (b'a' + (shift % 26) as u8) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(5, vec![0,0,0, 1,1] => 'a'; "example 1")]
    #[test_case(10, vec![0,1,0,1] => 'b'; "example 2")]
    #[test_case(14466811, vec![1,0,1,0,0,1,0,1,1,0,1,1,1,0,1,0,1,1,1,0,0,1,0,0,1] => 'g'; "case 1")]
    #[test_case(2, vec![1] => 'b'; "case 2")]
    #[test_case(1, vec![0,1] => 'a'; "case 3")]
    #[test_case(11, vec![0,0,1,1] => 'b'; "case 4")]
    fn test_solution(k: i64, operations: Vec<i32>) -> char {
        Solution::kth_character(k, operations)
    }
}
