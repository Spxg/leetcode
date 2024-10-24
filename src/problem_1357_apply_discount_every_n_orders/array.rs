struct Cashier {
    per: i32,
    nth: i32,
    discount: i32,
    prices: [i32; 201],
}

impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut map = [0; 201];
        for (product, price) in products.into_iter().zip(prices) {
            map[product as usize] = price;
        }
        Self {
            per: n,
            nth: 0,
            discount,
            prices: map,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut result = product.into_iter().zip(amount).fold(0., |acc, (p, a)| {
            f64::from(self.prices[p as usize]).mul_add(f64::from(a), acc)
        });
        self.nth = (self.nth + 1) % self.per;
        if self.nth == 0 {
            result = result * f64::from(100 - self.discount) / 100.0;
        }
        result
    }
}

impl super::Cashier for Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        Self::new(n, discount, products, prices)
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.get_bill(product, amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Cashier>();
    }
}
