pub struct Solution;

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut result = Vec::with_capacity(search_word.len());
        let mut products = products;
        products.sort_unstable();

        for prefix_end in 1..=search_word.len() {
            let prefix = &search_word[..prefix_end];
            let idx = products.partition_point(|product| product.as_str() < prefix);
            result.push(
                products[idx..]
                    .iter()
                    .take(3)
                    .take_while(|x| x.starts_with(prefix))
                    .map(Clone::clone)
                    .collect(),
            );
        }

        result
    }
}
