use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;

        let mut head = head.unwrap();
        let mut tail = Box::new(ListNode::new(0));
        let mut ptr1 = &mut head;
        let mut ptr2 = &mut tail;

        while let Some(mut even) = ptr1.next.take() {
            let odd = even.next.take();
            ptr2.next = Some(even);
            ptr2 = ptr2.next.as_mut().unwrap();
            if let Some(odd) = odd {
                ptr1.next = Some(odd);
                ptr1 = ptr1.next.as_mut().unwrap();
            } else {
                break;
            }
        }

        ptr1.next = tail.next.take();

        Some(head)
    }
}

impl super::Solution for Solution {
    fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::odd_even_list(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
