pub struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let bytes = colors.into_bytes();
        let n = bytes.len();
        if n <= 1 {
            return 0;
        }
        let mut total_removed = 0;
        let mut group_sum = needed_time[0];
        let mut group_max = needed_time[0];
        for i in 1..n {
            let time = needed_time[i];
            if bytes[i] == bytes[i - 1] {
                group_sum += time;
                if time > group_max {
                    group_max = time;
                }
            } else {
                total_removed += group_sum - group_max;
                group_sum = time;
                group_max = time;
            }
        }
        total_removed + group_sum - group_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abaac", vec![1,2,3,4,5] => 3; "example 1")]
    #[test_case("abc", vec![1,2,3] => 0; "example 2")]
    #[test_case("aabaa", vec![1,2,3,4,1] => 2; "example 3")]
    #[test_case("bbbaaa", vec![4,9,3,8,8,9] => 23; "case 1")]
    fn test_solution(colors: &str, needed_time: Vec<i32>) -> i32 {
        Solution::min_cost(String::from(colors), needed_time)
    }
}
