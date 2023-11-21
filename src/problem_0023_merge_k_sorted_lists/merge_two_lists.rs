use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn merge_two_lists(
            list1: Option<Box<ListNode>>,
            list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut lhs = list1;
            let mut rhs = list2;
            let mut result = None;
            let mut ref_result = &mut result;

            loop {
                match (&mut lhs, &mut rhs) {
                    (Some(node1), Some(node2)) => {
                        *ref_result = if node1.val <= node2.val {
                            let next = node1.next.take();
                            std::mem::replace(&mut lhs, next)
                        } else {
                            let next = node2.next.take();
                            std::mem::replace(&mut rhs, next)
                        };
                        ref_result = &mut ref_result.as_mut().unwrap().next;
                    }
                    (rest @ Some(_), None) | (None, rest @ Some(_)) => {
                        *ref_result = rest.take();
                    }
                    (None, None) => break result,
                }
            }
        }

        let mut lists = lists;
        loop {
            if lists.len() <= 1 {
                break lists.pop().flatten();
            }
            let node1 = lists.pop().flatten();
            let node2 = lists.pop().flatten();
            lists.push(merge_two_lists(node1, node2));
        }
    }
}

impl super::Solution for Solution {
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        Self::merge_k_lists(lists)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
