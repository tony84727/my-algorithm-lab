use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        fn gcf(mut a: i32, mut b: i32) -> i32 {
            loop {
                if a == b {
                    return a;
                }
                if a == 1 || b == 1 {
                    return 1;
                }
                if a > b {
                    a -= b;
                } else {
                    b -= a;
                }
            }
        }
        let mut connected = nums.clone();
        for n in nums {
            for c in connected.iter() {
                if gcf(n, *c) > 1 {
                    if n < *c {
                        for i in connected.iter_mut() {
                            if *i == *c {
                                *i = n;
                            }
                        }
                        break
                    }
                }
            }
        }
        let mut components: HashMap<i32, i32> = HashMap::new();
        for c in connected.into_iter() {
            *(components.entry(c).or_default()) += 1;
        }
        *components.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,6,15,35] => 4; "example 1")]
    fn test_solution(nums: Vec<i32>) -> i32 {
        Solution::largest_component_size(nums)
    }
}
