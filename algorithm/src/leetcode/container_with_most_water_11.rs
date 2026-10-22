pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut width = j - i;
        let mut max = 0;
        while width > 0 {
            let h1 = height[i];
            let h2 = height[j];
            let min_height = {
                if h1 > h2 {
                    h2
                } else {
                    h1
                }
            };
            let area = (width as i32) * min_height;
            if area > max {
                max = area;
            }
            if h1 > h2 {
                j -= 1
            } else {
                i += 1;
            }
            width = j - i;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,8,6,2,5,4,8,3,7] => 49; "example 1")]
    #[test_case(vec![1,1] => 1; "example 2")]
    #[test_case(vec![4,3,2,1,4] => 16; "example 3")]
    #[test_case(vec![2,3,10,5,7,8,9] => 36; "case 1")]
    fn test_solution(height: Vec<i32>) -> i32 {
        Solution::max_area(height)
    }
}
