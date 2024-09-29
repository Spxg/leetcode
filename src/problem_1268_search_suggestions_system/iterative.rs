pub struct Solution;

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut result = vec![vec![]; search_word.len()];
        let mut products = products;
        products.sort_unstable();

        for product in products {
            for (idx, _) in product
                .chars()
                .zip(search_word.chars())
                .take_while(|(x, y)| x == y)
                .enumerate()
            {
                if result[idx].len() == 3 {
                    continue;
                }
                result[idx].push(product.clone());
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        Self::suggested_products(products, search_word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
