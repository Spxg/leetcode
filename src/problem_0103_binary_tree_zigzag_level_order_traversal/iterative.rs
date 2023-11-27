pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut stack = vec![root];

        while !stack.is_empty() {
            let mut element = vec![];
            let mut nodes = std::mem::take(&mut stack);

            while let Some(node) = nodes.pop() {
                if let Some(node) = node {
                    element.push(node.borrow().val);
                    let left = node.borrow().left.clone();
                    let right = node.borrow().right.clone();

                    if result.len() % 2 == 0 {
                        stack.push(left);
                        stack.push(right);
                    } else {
                        stack.push(right);
                        stack.push(left);
                    }
                }
            }

            if !element.is_empty() {
                result.push(element);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Self::zigzag_level_order(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
