use crate::data_structures::ListNode;

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<i32>>();

        let mut result = 0;
        let mut head_ref = head.as_ref();
        let mut contains = false;

        while let Some(node) = head_ref {
            if set.contains(&node.val) {
                contains = true;
            } else {
                if contains {
                    result += 1;
                }
                contains = false;
            }
            head_ref = node.next.as_ref();
        }
        if contains {
            result += 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        Self::num_components(head, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
