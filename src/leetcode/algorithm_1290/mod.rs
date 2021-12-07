pub struct Solution;

use crate::leetcode::common::ListNode;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut sum = 0;
        let mut current = &head;
        while let Some(next) = current {
            sum *= 2;
            sum += next.val;
            current = &next.next;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,1] => 5; "example 1")]
    fn test_solution(head: Option<Box<ListNode>>) -> i32 {
        Solution::get_decimal_value(head)
    }
}
