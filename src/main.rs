mod store;
use crate::store::Store;

fn main() {
    let mut store = Store::new();

    store.set("test1".to_string(), "valueA".to_string(), None);
    store.set("test2".to_string(), "valueB".to_string(), None);
    store.set("test3".to_string(), "valueC".to_string(), None);

    let entry = store.get("test1");

    println!("{}", entry.unwrap().value())
}
