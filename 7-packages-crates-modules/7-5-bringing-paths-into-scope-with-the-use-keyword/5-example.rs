
//example 1

use rand::Rng;

fn main() {
  // Não, ao usar rand::Rng, o crate rand já está sendo acessado; não é necessário importar rand separadamente.
  let secret_number = rand::thread_rng().gen_range(1..=100);
}

// example 2
use std::collections::HashMap;

fn hash_example() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("{:?}", map);
}