use std::vec;

pub fn merge_sort(input: &[i32]) -> Vec<i32> {
    if input.len() <= 1 {
        return Vec::from(input);
    }
    let (left, right) = divide(input, input.len() / 2);
    let left = merge_sort(left);
    let right = merge_sort(right);
    combine(&left, &right)
}

fn divide(input: &[i32], index: usize) -> (&[i32], &[i32]) {
    input.split_at(index)
}

fn combine(a: &[i32], b: &[i32]) -> Vec<i32> {
    if a.is_empty() && b.is_empty() {
        return vec![];
    }
    if a.is_empty() {
        return Vec::from(b);
    }
    if b.is_empty() {
        return Vec::from(a);
    }
    let mut i = 0;
    let mut j = 0;
    let mut out = vec![];
    while i < a.len() || j < b.len() {
        let a = if i < a.len() { a[i] } else { i32::MAX };
        let b = if j < b.len() { b[j] } else { i32::MAX };
        out.push(if a > b {
            j += 1;
            b
        } else {
            i += 1;
            a
        })
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[4,3,2,1] => vec![1,2,3,4]; "reversed")]
    #[test_case(&[1,2,3,4,1,2,3,4] => vec![1,1,2,2,3,3,4,4]; "repeated")]
    #[test_case(&[0,9,5,4,1,2,3,7,8] => vec![0,1,2,3,4,5,7,8,9]; "case 1")]
    fn test_solution(input: &[i32]) -> Vec<i32> {
        merge_sort(input)
    }

    #[test]
    fn test_one() {
        assert_eq!(vec![1], merge_sort(&[1]));
    }

    #[test]
    fn test_two() {
        assert_eq!(vec![1, 2], merge_sort(&[2, 1]))
    }
}
