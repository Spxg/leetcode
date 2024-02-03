pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: &Rc<RefCell<TreeNode>>, depth: i32, result: &mut (i32, i32)) {
            match (&node.borrow().left, &node.borrow().right) {
                (None, None) => (),
                (Some(node), None) | (None, Some(node)) => {
                    if depth > result.0 {
                        result.0 = depth;
                        result.1 = node.borrow().val;
                    }
                    helper(node, depth + 1, result);
                }
                (Some(left), Some(right)) => {
                    if depth > result.0 {
                        result.0 = depth;
                        result.1 = left.borrow().val;
                    }
                    helper(right, depth + 1, result);
                    helper(left, depth + 1, result);
                }
            }
        }

        let mut result = (0, 0);
        if let Some(node) = &root {
            result.1 = node.borrow().val;
            helper(node, 1, &mut result);
        }
        result.1
    }
}

impl super::Solution for Solution {
    fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find_bottom_left_value(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
