pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn helper(node: &Rc<RefCell<TreeNode>>, result: &mut Option<String>, mut part: String) {
            let ch = (node.borrow().val as u8 + b'a') as char;
            part.push(ch);

            match (&node.borrow().left, &node.borrow().right) {
                (None, None) => {
                    if result.is_none()
                        || result.as_ref().is_some_and(|x| {
                            part.chars().rev().cmp(x.chars().rev()) == std::cmp::Ordering::Less
                        })
                    {
                        result.replace(part);
                    }
                }
                (None, Some(node)) | (Some(node), None) => helper(node, result, part),
                (Some(left), Some(right)) => {
                    helper(left, result, part.clone());
                    helper(right, result, part);
                }
            }
        }
        let mut result = None;
        helper(&root.unwrap(), &mut result, String::new());
        result.unwrap().chars().rev().collect()
    }
}

impl super::Solution for Solution {
    fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::smallest_from_leaf(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
