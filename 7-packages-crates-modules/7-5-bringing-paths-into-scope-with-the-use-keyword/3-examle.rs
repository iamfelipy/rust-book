// use paths idiomatico: funções
mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
}

// menos idiomatico
  use crate::front_of_house::hosting::add_to_waitlist;

  pub fn eat_at_restaurant() {
    add_to_waitlist();
  }

// use paths idiomatico: structs, enums
  // 1
  use std::collections::HashMap;

  fn main() {
      let mut map = HashMap::new();
      map.insert(1, 2);
  }
  // 2 - exceção
  // Nome do arquivo: src/lib.rs
  use std::fmt;
  use std::io;

  fn function1() -> fmt::Result {
      // --snip--
  }

  fn function2() -> io::Result<()> {
      // --snip--
  }