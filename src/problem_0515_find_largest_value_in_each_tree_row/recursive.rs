pub struct Solution;

use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<i32>) {
            if let Some(node) = node {
                let val = node.borrow().val;
                if result.len() == depth {
                    result.push(val);
                } else {
                    result[depth] = result[depth].max(val);
                }
                helper(&node.borrow().left, depth + 1, result);
                helper(&node.borrow().right, depth + 1, result);
            }
        }

        let mut result = vec![];
        helper(&root, 0, &mut result);
        result
    }
}

impl super::Solution for Solution {
    fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::largest_values(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
