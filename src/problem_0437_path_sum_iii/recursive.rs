pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn helper(
            root: &Option<Rc<RefCell<TreeNode>>>,
            sum: i64,
            target_sum: i64,
            map: &mut HashMap<i64, i32>,
            result: &mut i32,
        ) {
            if let Some(node) = root {
                let sum = sum + i64::from(node.borrow().val);
                if let Some(&count) = map.get(&(sum - target_sum)) {
                    *result += count;
                }
                *map.entry(sum).or_default() += 1;
                helper(&node.borrow().left, sum, target_sum, map, result);
                helper(&node.borrow().right, sum, target_sum, map, result);
                *map.get_mut(&sum).unwrap() -= 1;
            }
        }

        let mut result = 0;
        helper(
            &root,
            0,
            i64::from(target_sum),
            &mut HashMap::from([(0, 1)]),
            &mut result,
        );
        result
    }
}

impl super::Solution for Solution {
    fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
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
