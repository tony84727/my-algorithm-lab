pub struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut up = true;
        if arr[1] < arr[0] {
            return false;
        }
        for (i, num) in arr.iter().enumerate() {
            if i == arr.len() - 1 {
                continue;
            }
            let a = num;
            let b = &arr[i + 1];
            if a == b {
                return false;
            }
            if up {
                if b > a {
                    continue;
                }
                up = false;
                continue;
            }
            if b > a {
                return false;
            }
        }
        !up
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test_case(vec![2,1] => false; "example")]
    #[test_case(vec![3,5,5] => false; "case 1")]
    #[test_case(vec![2,0,2] => false; "case 2")]
    #[test_case(vec![0,1,2,3,4,5,6,7,8,9] => false; "case 3")]
    #[test_case(vec![9,8,7,6,5,4,3,2,1,0] => false; "case 4")]
    fn test_solution(arr: Vec<i32>) -> bool {
        Solution::valid_mountain_array(arr)
    }
}
