use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut nodes = vec![];
        let mut head = head;
        while let Some(mut node) = head.take() {
            nodes.push(node.val);
            head = node.next.take();
        }
        let mut iter = nodes.into_iter();
        while let Some(a) = iter.next() {
            if let Some(b) = iter.next_back() {
                if a != b {
                    return false;
                }
            } else {
                break;
            }
        }
        true
    }
}
