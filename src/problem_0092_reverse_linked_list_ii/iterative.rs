use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut reverse = None;
        let mut pointer = &mut head;

        for i in 1..=right {
            if left <= i {
                let mut node = pointer.take();
                *pointer = node.as_mut().and_then(|x| x.next.take());
                node.as_mut().unwrap().next = reverse;
                reverse = node;
            } else {
                pointer = &mut pointer.as_mut().unwrap().next;
            }
        }

        let tail = pointer.take();
        if let Some(mut node) = reverse.as_mut() {
            while node.next.is_some() {
                node = node.next.as_mut().unwrap();
            }
            node.next = tail;
        }

        if let Some(mut node) = head.as_mut() {
            while node.next.is_some() {
                node = node.next.as_mut().unwrap();
            }
            node.next = reverse;
            head
        } else {
            reverse
        }
    }
}

impl super::Solution for Solution {
    fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        Self::reverse_between(head, m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
