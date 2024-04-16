pub struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let secret = secret.into_bytes();
        let guess = guess.into_bytes();

        let mut cache = [0; 10];

        for (&s, &g) in secret.iter().zip(guess.iter()) {
            if s == g {
                bulls += 1;
            } else {
                cache[(s - b'0') as usize] += 1;
            }
        }

        for (s, g) in secret.into_iter().zip(guess) {
            if s == g {
                continue;
            }
            if cache[(g - b'0') as usize] > 0 {
                cows += 1;
            }
            cache[(g - b'0') as usize] -= 1;
        }

        format!("{bulls}A{cows}B")
    }
}

impl super::Solution for Solution {
    fn get_hint(secret: String, guess: String) -> String {
        Self::get_hint(secret, guess)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
