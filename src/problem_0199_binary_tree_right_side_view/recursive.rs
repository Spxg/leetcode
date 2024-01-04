pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Rc<RefCell<TreeNode>>, depth: usize, result: &mut Vec<i32>) {
            let val = node.borrow().val;

            if result.len() > depth {
                result[depth] = val;
            } else {
                result.push(val);
            }

            match (&node.borrow().left, &node.borrow().right) {
                (None, None) => (),
                (None, Some(node)) | (Some(node), None) => helper(node, depth + 1, result),
                (Some(left), Some(right)) => {
                    helper(left, depth + 1, result);
                    helper(right, depth + 1, result);
                }
            }
        }

        let mut result = vec![];
        if let Some(node) = root {
            helper(&node, 0, &mut result);
        }
        result
    }
}

impl super::Solution for Solution {
    fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::right_side_view(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
