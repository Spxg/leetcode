pub mod iterative;
pub mod std;

pub trait Solution {
    fn reverse_bits(n: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(43_261_596, 964_176_192), (4_294_967_293, 3_221_225_471)];

        for (n, expected) in test_cases {
            assert_eq!(S::reverse_bits(n), expected);
        }
    }
}
