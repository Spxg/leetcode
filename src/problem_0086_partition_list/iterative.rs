use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = None;
        let mut head_pointer = &mut head;
        let mut tail_pointer = &mut tail;

        while let Some(mut node) = head_pointer.take() {
            if node.val < x {
                *head_pointer = Some(node);
                head_pointer = &mut head_pointer.as_mut().unwrap().next;
            } else {
                *head_pointer = node.next.take();
                *tail_pointer = Some(node);
                tail_pointer = &mut tail_pointer.as_mut().unwrap().next;
            }
        }

        *head_pointer = tail;
        head
    }
}

impl super::Solution for Solution {
    fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        Self::partition(head, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
