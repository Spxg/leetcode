pub struct Solution;

use crate::data_structures::{ListNode, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            (!nums.is_empty()).then(|| {
                let mid = nums.len() / 2;
                Rc::new(RefCell::new(TreeNode {
                    val: nums[mid],
                    left: helper(&nums[0..mid]),
                    right: helper(&nums[mid + 1..]),
                }))
            })
        }

        let mut head = head;
        let mut array = vec![];
        while let Some(node) = head {
            array.push(node.val);
            head = node.next;
        }

        helper(&array)
    }
}

impl super::Solution for Solution {
    fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sorted_list_to_bst(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
