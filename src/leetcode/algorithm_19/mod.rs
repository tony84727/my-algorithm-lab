use crate::leetcode::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut lead = &head.clone();
        let mut lag = &mut head;
        for _ in 0..n {
            lead = match lead {
                Some(node) => &node.next,
                None => return head,
            }
        }
        while let Some(node) = lead {
            lead = &node.next;
            lag = match lag {
                Some(next) => &mut next.next,
                None => break,
            };
        }
        *lag = lag.take().and_then(|x| x.next);
        head
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    use crate::leetcode::common::{list_to_vec, vec_to_list, ListNode};

    #[test_case(vec![1,2,3,4,5], 2 => vec![1,2,3,5]; "example 1")]
    #[test_case(vec![1],1 => Vec::<i32>::new(); "example 2")]
    #[test_case(vec![1,2], 1 => vec![1]; "example 3")]
    fn test_solution(items: Vec<i32>, n: i32) -> Vec<i32> {
        let list = vec_to_list(items);
        list_to_vec(Solution::remove_nth_from_end(list, n))
    }
}
