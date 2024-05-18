use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let len = std::iter::successors(head.as_ref(), |node| node.next.as_ref()).count();
        let mut head = head;
        let mut reversed = None;
        for _ in 0..len / 2 {
            let mut node = head.unwrap();
            head = std::mem::replace(&mut node.next, reversed);
            reversed = Some(node);
        }
        if len % 2 != 0 {
            head = head.unwrap().next;
        }
        reversed == head
    }
}

impl super::Solution for Solution {
    fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        Self::is_palindrome(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
