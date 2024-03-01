pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, num: &mut Option<i32>, result: &mut i32) {
            if let Some(node) = node {
                helper(&node.borrow().left, num, result);
                let value = node.borrow().val;
                if let Some(v) = num.take() {
                    *result = (*result).min(value - v);
                }
                *num = Some(value);
                helper(&node.borrow().right, num, result);
            }
        }

        let mut result = i32::MAX;
        helper(&root, &mut None, &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_minimum_difference(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
