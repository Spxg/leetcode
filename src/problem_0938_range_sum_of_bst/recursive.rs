pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        #[allow(clippy::ref_option)]
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
            root.as_ref().map_or(0, |node| {
                let val = node.borrow().val;
                if (low..=high).contains(&val) {
                    val + helper(&node.borrow().right, low, high)
                        + helper(&node.borrow().left, low, high)
                } else if val < low {
                    helper(&node.borrow().right, low, high)
                } else {
                    helper(&node.borrow().left, low, high)
                }
            })
        }

        helper(&root, low, high)
    }
}

impl super::Solution for Solution {
    fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Self::range_sum_bst(root, low, high)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
