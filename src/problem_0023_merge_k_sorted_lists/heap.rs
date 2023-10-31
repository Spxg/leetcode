use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = std::collections::BinaryHeap::new();
        for mut list in lists {
            while let Some(node) = list {
                heap.push(node.val);
                list = node.next;
            }
        }

        let mut node = None;
        while let Some(val) = heap.pop() {
            node = Some(Box::new(ListNode { val, next: node }));
        }

        node
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
