pub fn insertion_sort(mut input: Vec<i32>) -> Vec<i32> {
    for i in 1..input.len() {
        let value = input[i];
        let mut j = (i as i64) - 1;
        while j >= 0 && input[j as usize] >= value {
            input[(j + 1) as usize] = input[j as usize];
            j -= 1;
        }
        input[(j + 1) as usize] = value;
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn is_sorted(input: &[i32]) -> bool {
        if input.len() <= 1 {
            return true;
        }
        for i in 1..input.len() {
            if input[i] < input[i - 1] {
                return false;
            }
        }
        true
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test_case(vec![1,2,3,4] => true; "case 1")]
    #[test_case(vec![1,3,2,4] => false; "case 2")]
    fn test_is_sorted(input: Vec<i32>) -> bool {
        is_sorted(&input)
    }

    #[test_case(vec![4,3,2,1] => vec![1,2,3,4]; "reversed")]
    #[test_case(vec![1,2,3,4,1,2,3,4] => vec![1,1,2,2,3,3,4,4]; "repeated")]
    #[test_case(vec![0,9,5,4,1,2,3,7,8] => vec![0,1,2,3,4,5,7,8,9]; "case 1")]
    fn test_insertion_sort(input: Vec<i32>) -> Vec<i32> {
        insertion_sort(input)
    }
}
