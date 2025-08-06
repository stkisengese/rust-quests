pub mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> Store {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .max_by_key(|store| store.square_meters)
        .cloned()
        .expect("Mall should have at least one store")
}
