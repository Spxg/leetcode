pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = Vec::from([root.unwrap()]);
        let mut iter = voyage.into_iter();
        let mut val = iter.next();

        while let Some(node) = stack.pop() {
            let node_val = node.borrow().val;
            if val.is_some_and(|x| x != node_val) {
                return vec![-1];
            }
            val = iter.next();

            let mut node_mut = node.borrow_mut();
            match (node_mut.left.take(), node_mut.right.take()) {
                (None, None) => continue,
                (Some(node), None) | (None, Some(node)) => stack.push(node),
                (Some(left), Some(right)) => {
                    if val.is_some_and(|x| x == left.borrow().val) {
                        stack.push(right);
                        stack.push(left);
                    } else {
                        result.push(node_val);
                        stack.push(left);
                        stack.push(right);
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        Self::flip_match_voyage(root, voyage)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
