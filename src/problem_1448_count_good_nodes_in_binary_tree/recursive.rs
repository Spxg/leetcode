pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        #[allow(clippy::ref_option)]
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, max: i32, result: &mut i32) {
            let Some(node) = node else { return };
            let val = node.borrow().val;
            if val >= max {
                *result += 1;
            }
            helper(&node.borrow().left, max.max(val), result);
            helper(&node.borrow().right, max.max(val), result);
        }
        let mut result = 0;
        helper(&root, i32::MIN, &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::good_nodes(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
