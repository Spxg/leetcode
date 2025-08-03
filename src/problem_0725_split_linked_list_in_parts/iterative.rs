use crate::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;
        let mut head = head;

        let mut result = Vec::with_capacity(k);
        let count = std::iter::successors(head.as_ref(), |x| x.next.as_ref()).count();
        let size = count / k;

        for size in
            std::iter::repeat_n(size + 1, count % k).chain(std::iter::repeat_n(size, k - count % k))
        {
            if size == 0 {
                result.push(None);
            } else {
                let mut pointer = head.as_mut();
                for _ in 0..size - 1 {
                    if let Some(p) = pointer {
                        pointer = p.next.as_mut();
                    }
                }
                let new_head = pointer.unwrap().next.take();
                result.push(head);
                head = new_head;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        Self::split_list_to_parts(head, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
