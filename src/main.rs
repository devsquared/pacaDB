mod store;
use std::sync::{Arc, Mutex};

use crate::store::Store;

fn main() {
    let store = Arc::new(Mutex::new(Store::new()));
    let store_one = Arc::clone(&store);
    let store_two = Arc::clone(&store);

    let handle_one = std::thread::spawn(move || {
        store_one
            .lock()
            .unwrap()
            .set("test1".to_string(), "valueA".to_string(), None);
        store_one
            .lock()
            .unwrap()
            .set("test2".to_string(), "valueB".to_string(), None);
    });

    let handle_two = std::thread::spawn(move || {
        store_two
            .lock()
            .unwrap()
            .set("testA".to_string(), "value1".to_string(), None);
        store_two
            .lock()
            .unwrap()
            .set("testB".to_string(), "value2".to_string(), None);
    });

    handle_one.join().unwrap();
    handle_two.join().unwrap();

    let s = store.lock().unwrap();
    println!("{}", s.get("test1").unwrap().value());
    println!("{}", s.get("test2").unwrap().value());
    println!("{}", s.get("testA").unwrap().value());
    println!("{}", s.get("testB").unwrap().value());
}
