pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(prev: i32, node: &Rc<RefCell<TreeNode>>, result: &mut i32) {
            let val = prev * 2 + node.borrow().val;
            match (&node.borrow().left, &node.borrow().right) {
                (None, None) => *result += val,
                (None, Some(right)) => helper(val, right, result),
                (Some(left), None) => helper(val, left, result),
                (Some(left), Some(right)) => {
                    helper(val, left, result);
                    helper(val, right, result);
                }
            }
        }
        let mut result = 0;
        helper(0, &root.unwrap(), &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_root_to_leaf(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
