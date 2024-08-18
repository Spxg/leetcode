pub struct Solution;

use crate::data_structures::ListNode;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let length = std::iter::successors(head.as_ref(), |x| x.next.as_ref()).count();
        let mut result = head;
        for _ in 0..length / 2 {
            result = result.and_then(|x| x.next);
        }
        result
    }
}

impl super::Solution for Solution {
    fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::middle_node(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
