use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode { next: head, val: 0 });
        let mut curr = &mut head;
        while let Some(node) = curr.next.as_mut() {
            if node.val == val {
                let next = node.next.take();
                curr.next = next;
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }
        head.next
    }
}

impl super::Solution for Solution {
    fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        Self::remove_elements(head, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
