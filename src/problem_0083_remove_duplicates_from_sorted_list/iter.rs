use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev_pointer = &mut head;

        while let Some(prev) = prev_pointer {
            let mut next = prev.next.take();
            while next.as_ref().map(|x| x.val) == Some(prev.val) {
                next = next.and_then(|x| x.next);
            }
            prev.next = next;
            prev_pointer = &mut prev.next;
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
