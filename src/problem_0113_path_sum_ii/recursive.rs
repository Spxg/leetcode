pub struct Solution;

use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn helper(
            root: &Rc<RefCell<TreeNode>>,
            target_sum: i32,
            sum: i32,
            ele: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            let val = root.borrow().val;
            let current_sum = sum + val;
            ele.push(val);

            match (&root.borrow().left, &root.borrow().right) {
                (None, None) => {
                    if target_sum == current_sum {
                        result.push(ele.clone());
                    }
                }
                (None, Some(right)) => helper(right, target_sum, current_sum, ele, result),
                (Some(left), None) => helper(left, target_sum, current_sum, ele, result),
                (Some(left), Some(right)) => {
                    helper(left, target_sum, current_sum, ele, result);
                    helper(right, target_sum, current_sum, ele, result);
                }
            }

            ele.pop();
        }

        let mut result = vec![];
        if let Some(root) = root {
            helper(&root, target_sum, 0, &mut Vec::new(), &mut result);
        }
        result
    }
}

impl super::Solution for Solution {
    fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        Self::path_sum(root, sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
