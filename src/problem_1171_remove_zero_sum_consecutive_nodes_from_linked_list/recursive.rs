pub struct Solution;

use crate::data_structures::ListNode;

use std::collections::HashMap;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn helper(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut map = HashMap::new();
            let mut sum = 0;
            let mut range = None;

            for (nth, node) in std::iter::successors(head.as_ref(), |n| n.next.as_ref()).enumerate()
            {
                map.entry(sum).or_insert(nth);
                sum += node.val;
                if let Some(&start) = map.get(&sum) {
                    range = Some((start, nth));
                    break;
                }
            }

            let Some((start, end)) = range else {
                return head;
            };

            let mut head = Box::new(ListNode { val: 0, next: head });
            let mut head_mut = head.as_mut();
            for _ in 0..start {
                head_mut = head_mut.next.as_mut().unwrap();
            }

            let mut tail = head_mut.next.take();
            for _ in 0..=(end - start) {
                tail = tail.as_mut().and_then(|x| x.next.take());
            }

            head_mut.next = tail;

            helper(head.next.take())
        }

        helper(head)
    }
}

impl super::Solution for Solution {
    fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::remove_zero_sum_sublists(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
