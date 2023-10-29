use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut pointer = &mut head;

        while let Some(mut prev) = pointer.take() {
            let mut next = prev.next.take();
            if next.is_some() {
                prev.next = next.as_mut().and_then(|x| x.next.take());
                next.as_mut().unwrap().next = Some(prev);
                *pointer = next;
            } else {
                *pointer = Some(prev);
                break;
            }

            pointer = &mut pointer.as_mut().unwrap().next;
            pointer = &mut pointer.as_mut().unwrap().next;
        }

        head
    }
}

impl super::Solution for Solution {
    fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::swap_pairs(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
