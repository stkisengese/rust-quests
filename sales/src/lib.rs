#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart { Cart::default() }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
	for product in s.products.iter() {
	    if product.0 == ele {
	    	self.items.push(product.clone());
	    }
	}
   }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
	 self.receipt = self.items.iter().map(|item| item.1).collect();
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let free = self.receipt.len() / 3;
        let total: f32 = self.receipt.iter().sum();
        let discount = self.receipt.iter().take(free).sum::<f32>();
        let total_discount = total - discount;
        let discount_per_item = (total_discount * 100.0 / total) / 100.0;
        self.receipt.iter_mut().for_each(|price| {
            *price = ((*price * discount_per_item * 100.0).round()) / 100.0;
        });
        self.receipt.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    	let store = Store::new(vec![
        	(String::from("product A"), 1.23),
        	(String::from("product B"), 23.1),
        	(String::from("product C"), 3.12)]);

    	println!("{:?}", store);

    	let mut cart = Cart::new();
    	cart.insert_item(&store, String::from("product A"));
    	cart.insert_item(&store, String::from("product B"));
    	cart.insert_item(&store, String::from("product C"));

	let result = cart.generate_receipt();

    	println!("{:?}", cart);
        
        assert_eq!(result, [1.17, 2.98, 22.06]);
    }
}
