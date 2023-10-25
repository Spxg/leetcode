use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        head.as_ref()?;

        let len = std::iter::successors(head.as_ref(), |node| node.next.as_ref()).count();
        let nth = len - k as usize % len;

        let mut head = head;
        let mut ref_node = &mut head;

        for _ in 1..nth {
            ref_node = &mut ref_node.as_mut().unwrap().next;
        }

        if let Some(new_head) = ref_node.as_mut().unwrap().next.take() {
            let mut new_head = Some(new_head);
            let mut ref_head = &mut new_head;
            while ref_head.as_ref().unwrap().next.is_some() {
                ref_head = &mut ref_head.as_mut().unwrap().next;
            }
            ref_head.as_mut().unwrap().next = head;
            new_head
        } else {
            head
        }
    }
}

impl super::Solution for Solution {
    fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        Self::rotate_right(head, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
