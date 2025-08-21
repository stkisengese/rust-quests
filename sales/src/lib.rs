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
    pub receipt: Vec<Store>,
}

impl Cart {
    pub fn new() -> Cart { Cart::default() }
    pub fn insert_item(&mut self, s: &Store, ele: String) {}
    pub fn generate_receipt(&mut self) -> Vec<f32> {}
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
