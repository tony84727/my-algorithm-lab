pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut remaining = pushed;
        remaining.reverse();
        let mut stack = vec![];
        for p in popped.into_iter() {
            if let Some(head) = stack.last() {
                if head == &p {
                    stack.pop();
                    continue;
                }
            }
            while let Some(new) = remaining.pop() {
                if new == p {
                    break;
                }
                stack.push(new);
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,4,5], vec![4,5,3,2,1] => true; "example 1")]
    #[test_case(vec![1,2,3,4,5], vec![4,3,5,1,2] => false; "example 2")]
    fn test_solution(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        Solution::validate_stack_sequences(pushed, popped)
    }
}
