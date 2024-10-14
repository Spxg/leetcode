pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(
            node: &Rc<RefCell<TreeNode>>,
            parent: Option<i32>,
            grandparent: Option<i32>,
        ) -> i32 {
            let mut result = 0;
            let val = node.borrow().val;
            if grandparent.is_some_and(|x| x % 2 == 0) {
                result += val;
            }
            if let Some(left) = &node.borrow().left {
                result += helper(left, Some(val), parent);
            }
            if let Some(right) = &node.borrow().right {
                result += helper(right, Some(val), parent);
            }
            result
        }
        helper(&root.unwrap(), None, None)
    }
}

impl super::Solution for Solution {
    fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_even_grandparent(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
