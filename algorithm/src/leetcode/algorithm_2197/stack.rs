pub struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        for n in nums.into_iter() {
            let mut current = n as i64;
            while let Some(&existing) = stack.last() {
                let gcd = Self::gcd(current, existing);
                if gcd == 1 {
                    break;
                }
                stack.pop();
                current = current * existing / gcd;
            }
            stack.push(current);
        }
        stack.into_iter().map(|x| x as i32).collect()
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while a != 0 && b != 0 {
            (a, b) = (a.max(b), a.min(b));
            a %= b;
        }
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![6,4,3,2,7,6,2] => vec![12,7,6]; "example 1")]
    #[test_case(vec![2,2,1,1,3,3,3] => vec![2,1,1,3]; "example 2")]
    #[test_case(vec![517,11,121,517,3,51,3,1887,5] => vec![5687,1887,5]; "case 1")]
    #[test_case(vec![5687,3,1887,5] => vec![5687,1887,5]; "case 2")]
    #[test_case(vec![31,97561,97561,97561,97561,97561,97561,97561,97561] => vec![31,97561]; "case 3")]
    #[test_case(vec![287,41,49,287,899,23,23,20677,5,825] => vec![2009,20677,825]; "case 4")]
    fn test_solution(nums: Vec<i32>) -> Vec<i32> {
        Solution::replace_non_coprimes(nums)
    }
}
