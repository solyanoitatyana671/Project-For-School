use std::collections::HashMap;
use rand::prelude::*;
use std::iter::Iterator;

fn main() {
    let mut rng = thread_rng();
    let mut map: HashMap<i32, i32> = HashMap::new();
    for _ in 0..100 {
        let key = rng.gen_range(0..10);
        if !map.contains_key(&key) {
            map.insert(key, 0);
        }
        *map.get_mut(&key).unwrap() += 1;
    }
    for (key, value) in &map {
        println!("{} {}", key, value);
    }
}
