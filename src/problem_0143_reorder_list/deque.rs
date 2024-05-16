use crate::data_structures::ListNode;

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut deque = VecDeque::new();

        let mut prev_head = head.take();
        while let Some(mut node) = prev_head {
            prev_head = node.next.take();
            deque.push_back(node);
        }

        let mut new_head = Box::new(ListNode::new(0));
        let mut pointer = &mut new_head;

        while !deque.is_empty() {
            if let Some(node) = deque.pop_front() {
                pointer.next = Some(node);
                pointer = pointer.next.as_mut().unwrap();
            }
            if let Some(node) = deque.pop_back() {
                pointer.next = Some(node);
                pointer = pointer.next.as_mut().unwrap();
            }
        }

        *head = new_head.next.take();
    }
}

impl super::Solution for Solution {
    fn reorder_list(head: &mut Option<Box<ListNode>>) {
        Self::reorder_list(head);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
