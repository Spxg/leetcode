pub struct Solution;

use crate::data_structures::ListNode;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut result = ListNode::new(0);
        let mut ref_result = &mut result;

        loop {
            match (&mut l1, &mut l2) {
                (Some(lhs), Some(rhs)) => {
                    let sum = lhs.val + rhs.val + carry;
                    carry = sum / 10;
                    ref_result.next = Some(Box::new(ListNode::new(sum % 10)));
                    ref_result = ref_result.next.as_mut().unwrap();
                }
                (Some(node), None) | (None, Some(node)) => {
                    let sum = node.val + carry;
                    carry = sum / 10;
                    ref_result.next = Some(Box::new(ListNode::new(sum % 10)));
                    ref_result = ref_result.next.as_mut().unwrap();
                }
                (None, None) => {
                    if carry == 0 {
                        break;
                    }
                    ref_result.next = Some(Box::new(ListNode::new(carry)));
                    break;
                }
            }
            l1 = l1.and_then(|mut x| x.next.take());
            l2 = l2.and_then(|mut x| x.next.take());
        }

        result.next
    }
}

impl super::Solution for Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
