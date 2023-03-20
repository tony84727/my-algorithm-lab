pub struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 {
            n = 1;
        }
        for i in 0..flowerbed.len() {
            let next = i != flowerbed.len() - 1 && flowerbed[i + 1] == 1;
            let last = i != 0 && flowerbed[i - 1] == 1;
            if flowerbed[i] == 0 && !next && !last {
                n -= 1;
                flowerbed[i] = 1;
            }
            if n == 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,0,0,1], 1 => true; "example 1")]
    #[test_case(vec![1,0,0,0,1], 2 => false; "example 2")]
    #[test_case(vec![1,0,1,0,1,0,1], 0 => false; "case 1")]
    #[test_case(vec![0,0,0,0,0,1,0,0],0 => true; "case 2")]
    fn test_solution(flowerbed: Vec<i32>, n: i32) -> bool {
        Solution::can_place_flowers(flowerbed, n)
    }
}
