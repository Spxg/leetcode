use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let count = std::iter::successors(head.as_ref(), |node| node.next.as_ref()).count();
        let nth = count - n as usize;

        let mut head = head;
        let mut ref_node = &mut head;

        for _ in 0..nth {
            ref_node = &mut ref_node.as_mut().unwrap().next;
        }

        *ref_node = ref_node.as_mut().unwrap().next.take();

        head
    }
}

impl super::Solution for Solution {
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_nth_from_end(head, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
