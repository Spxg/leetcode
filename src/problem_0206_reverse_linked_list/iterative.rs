use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut new_head = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = new_head;
            new_head = Some(node);
        }
        new_head
    }
}

impl super::Solution for Solution {
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_list(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
