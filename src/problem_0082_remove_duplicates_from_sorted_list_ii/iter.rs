use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut pointer = &mut head;

        while let Some(prev) = pointer {
            if Some(prev.val) == prev.next.as_ref().map(|x| x.val) {
                let mut next = prev.next.take();
                while Some(prev.val) == next.as_ref().map(|x| x.val) {
                    next = next.and_then(|x| x.next);
                }
                *pointer = next;
            } else {
                pointer = &mut pointer.as_mut().unwrap().next;
            }
        }

        head
    }
}

impl super::Solution for Solution {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::delete_duplicates(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
