pub struct Solution;

impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut memory1 = memory1;
        let mut memory2 = memory2;

        for crash in 1.. {
            if memory1.max(memory2) < crash {
                return vec![crash, memory1, memory2];
            }
            match memory1.cmp(&memory2) {
                std::cmp::Ordering::Less => memory2 -= crash,
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => memory1 -= crash,
            }
        }

        unreachable!()
    }
}

impl super::Solution for Solution {
    fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        Self::mem_leak(memory1, memory2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
